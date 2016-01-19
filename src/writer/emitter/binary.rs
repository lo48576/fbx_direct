//! Contains implementation of Binary FBX emitter.

extern crate byteorder;
extern crate flate2;

use std::io::{Write, Seek, SeekFrom};
use self::byteorder::{LittleEndian, WriteBytesExt};
use writer::error::{Result, Error};
use common::Property;

/// A writer for Binary FBX.
#[derive(Debug, Clone)]
pub struct BinaryEmitter {
    version: u32,
    pos: u64,
    end_offset_pos_stack: Vec<u64>,
    null_record_necessities: Vec<bool>,
}

impl BinaryEmitter {
    /// Constructs Binary FBX writer with FBX version.
    pub fn new(version: u32) -> Self {
        BinaryEmitter {
            version: version,
            pos: 0,
            end_offset_pos_stack: vec![],
            null_record_necessities: vec![],
        }
    }

    pub fn emit_start_fbx<W: Write + Seek>(&mut self, sink: &mut W, ver: u32) -> Result<()> {
        if (ver < 7000) || (ver >= 8000) {
            error!("Unsupported version: {}", ver);
            return Err(Error::UnsupportedFbxVersion(ver));
        }
        // Write magic binary for Binary FBX.
        try!(sink.write(b"Kaydara FBX Binary  \x00"));
        // Meaning is unknown, but value seems to be always `[0x1A, 0x00]`.
        try!(sink.write(b"\x1a\x00"));
        // Write FBX version.
        try!(sink.write_u32::<LittleEndian>(ver));

        Ok(())
    }

    pub fn emit_end_fbx<W: Write + Seek>(&mut self, sink: &mut W) -> Result<()> {
        // Write null record header.
        if self.version < 7500 {
            // 13: size of a node record header (4+4+4+1).
            try!(sink.write_all(&[0; 13]));
        } else {
            // 25: size of a node record header (8+8+8+1).
            try!(sink.write_all(&[0; 25]));
        }

        // Write footer.

        // Write unknown footer.
        // NOTE: This footer is `fa bc ax 0x dx cx dx 6x bx 7x fx 8x 1x fx 2x 7x`,
        //       but detail is unknown.
        try!(sink.write_all(&[
           0xfa as u8, 0xbc, 0xaf, 0x0f,
           0xdf, 0xcf, 0xdf, 0x6f,
           0xbf, 0x7f, 0xff, 0x8f,
           0x1f, 0xff, 0x2f, 0x7f
        ]));
        // Write padding.
        {
            let current_off = try!(sink.seek(SeekFrom::Current(0))) & 0x0f;
            if current_off != 0 {
                try!(sink.write_all(&(current_off..16).map(|_| 0).collect::<Vec<u8>>()));
            }
        }
        // Write `0u32`, FBX version, and [0; 120].
        try!(sink.write_all(&[0; 4]));
        try!(sink.write_u32::<LittleEndian>(self.version));
        try!(sink.write_all(&[0; 120]));
        // Write unknown but fixed magic.
        try!(sink.write_all(&[
            0xf8 as u8, 0x5a, 0x8c, 0x6a,
            0xde, 0xf5, 0xd9, 0x7e,
            0xec, 0xe9, 0x0c, 0xe3,
            0x75, 0x8f, 0x29, 0x0b
        ]));

        // All done.
        Ok(())
    }

    pub fn emit_start_node<W: Write + Seek>(&mut self, sink: &mut W, name: &str, properties: &[Property]) -> Result<()> {
        if let Some(top) = self.null_record_necessities.last_mut() {
            // Parent node requires null record, because it has child node (the current node!).
            *top = true;
        }
        self.null_record_necessities.push(properties.is_empty());

        // Write node record header.
        // For detail of node record header, see `reader::parser::binary::NodeRecordHeader` struct.
        let prop_list_len_offset;
        if self.version < 7500 {
            // Write a placeholder for `end_offset` and remember current offset.
            self.end_offset_pos_stack.push(try!(sink.seek(SeekFrom::Current(0))));
            try!(sink.write_u32::<LittleEndian>(0xef_be_ad_de));
            // Write `num_properties`.
            if properties.len() > u32::max_value() as usize {
                return Err(Error::DataTooLarge(format!("Number of node properties ({}) is too large for FBX {}", properties.len(), self.version)));
            }
            try!(sink.write_u32::<LittleEndian>(properties.len() as u32));
            // Write a default value of `property_list_len`.
            prop_list_len_offset = try!(sink.seek(SeekFrom::Current(0)));
            try!(sink.write_u32::<LittleEndian>(0));
        } else {
            // Write a placeholder for `end_offset` and remember current offset.
            self.end_offset_pos_stack.push(try!(sink.seek(SeekFrom::Current(0))));
            try!(sink.write_u64::<LittleEndian>(0xef_be_ad_de_ef_be_ad_de));
            // Write `num_properties`.
            if properties.len() > u64::max_value() as usize {
                return Err(Error::DataTooLarge(format!("Number of node properties ({}) is too large for FBX {}", properties.len(), self.version)));
            }
            try!(sink.write_u64::<LittleEndian>(properties.len() as u64));
            // Write a default value of `property_list_len`.
            prop_list_len_offset = try!(sink.seek(SeekFrom::Current(0)));
            try!(sink.write_u64::<LittleEndian>(0));
        }
        // Write length of the node name.
        try!(sink.write_u8(name.len() as u8));

        // Write a node name.
        try!(sink.write_all(name.as_bytes()));

        // Write properties.
        if !properties.is_empty() {
            let mut props_byte_size = 0_u64;
            for prop in properties {
                macro_rules! read_array_value {
                    ($vec:ident, $type_code:expr, $elem_type_writer:ident) => ({
                        try!(sink.write_u8($type_code as u8));

                        // Write a property array header.
                        // Write array length (element numbers, not byte size).
                        try!(sink.write_u32::<LittleEndian>($vec.len() as u32));
                        // Write encoding.
                        // 0 for plain data, 1 for zlib-compressed data.
                        try!(sink.write_u32::<LittleEndian>(1));
                        // Write a placeholder for byte size of properties.
                        let byte_size_pos = try!(sink.seek(SeekFrom::Current(0)));
                        try!(sink.write_u32::<LittleEndian>(0));

                        let vec_start_pos = try!(sink.seek(SeekFrom::Current(0)));
                        {
                            let mut encoder = flate2::write::ZlibEncoder::new(sink.by_ref(), flate2::Compression::Default);
                            for &v in $vec {
                                //try!(encoder.write_i32::<LittleEndian>(v));
                                try!(encoder.$elem_type_writer::<LittleEndian>(v));
                            }
                            try!(encoder.finish());
                        }
                        let last_pos = try!(sink.seek(SeekFrom::Current(0)));

                        // Update byte size of properties.
                        let byte_size = last_pos - vec_start_pos;
                        try!(sink.seek(SeekFrom::Start(byte_size_pos)));
                        try!(sink.write_u32::<LittleEndian>(byte_size as u32));
                        try!(sink.seek(SeekFrom::Start(last_pos)));
                        // 12: property array header.
                        12 + byte_size as u64
                    })
                };
                props_byte_size += 1 + match *prop {
                    Property::Bool(v) => {
                        try!(sink.write_u8('C' as u8));
                        // `'Y'` is `0x59`,  `'T'` is `0x54`.
                        try!(sink.write_u8(if v { 'Y' } else { 'T' } as u8));
                        1
                    },
                    Property::I16(v) => {
                        try!(sink.write_u8('Y' as u8));
                        try!(sink.write_i16::<LittleEndian>(v));
                        2
                    },
                    Property::I32(v) => {
                        try!(sink.write_u8('I' as u8));
                        try!(sink.write_i32::<LittleEndian>(v));
                        4
                    },
                    Property::I64(v) => {
                        try!(sink.write_u8('L' as u8));
                        try!(sink.write_i64::<LittleEndian>(v));
                        8
                    },
                    Property::F32(v) => {
                        try!(sink.write_u8('F' as u8));
                        try!(sink.write_f32::<LittleEndian>(v));
                        4
                    },
                    Property::F64(v) => {
                        try!(sink.write_u8('D' as u8));
                        try!(sink.write_f64::<LittleEndian>(v));
                        8
                    },
                    Property::VecBool(vec) => {
                        try!(sink.write_u8('b' as u8));
                        for v in vec.iter().map(|&v| if v { 'Y' } else { 'T' } as u8) {
                            try!(sink.write_u8(v));
                        }
                        vec.len() as u64
                    },
                    Property::VecI32(vec) => {
                        read_array_value!(vec, 'i', write_i32)
                    },
                    Property::VecI64(vec) => {
                        read_array_value!(vec, 'l', write_i64)
                    },
                    Property::VecF32(vec) => {
                        read_array_value!(vec, 'f', write_f32)
                    },
                    Property::VecF64(vec) => {
                        read_array_value!(vec, 'd', write_f64)
                    },
                    Property::String(s) => {
                        try!(sink.write_u8('S' as u8));
                        try!(sink.write_u32::<LittleEndian>(s.len() as u32));
                        try!(sink.write_all(s.as_bytes()));
                        4 + s.len() as u64
                    },
                    Property::Binary(b) => {
                        try!(sink.write_u8('R' as u8));
                        try!(sink.write_u32::<LittleEndian>(b.len() as u32));
                        try!(sink.write_all(b));
                        4 + b.len() as u64
                    },
                };
            }
            // Update `property_list_len`
            let last_pos = try!(sink.seek(SeekFrom::Current(0)));
            try!(sink.seek(SeekFrom::Start(prop_list_len_offset)));
            if self.version < 7500 {
                if props_byte_size > u32::max_value() as u64 {
                    return Err(Error::DataTooLarge(format!("Properties size ({} bytes) is too large for FBX {}", props_byte_size, self.version)));
                }
                try!(sink.write_u32::<LittleEndian>(props_byte_size as u32));
            } else {
                try!(sink.write_u64::<LittleEndian>(props_byte_size));
            }
            try!(sink.seek(SeekFrom::Start(last_pos)));
        }

        Ok(())
    }

    pub fn emit_end_node<W: Write + Seek>(&mut self, sink: &mut W) -> Result<()> {
        // Write a null record header if necessary.
        if let Some(required) = self.null_record_necessities.pop() {
            if required {
                if self.version < 7500 {
                    // 13: size of a node record header (4+4+4+1).
                    try!(sink.write_all(&[0; 13]));
                } else {
                    // 25: size of a node record header (8+8+8+1).
                    try!(sink.write_all(&[0; 25]));
                }
            }
        } else {
            return Err(Error::ExtraEndNode);
        }

        // Update `end_offset`.
        let last_pos = try!(sink.seek(SeekFrom::Current(0)));
        try!(sink.seek(SeekFrom::Start(self.end_offset_pos_stack.pop().unwrap())));
        if self.version < 7500 {
            if last_pos > u32::max_value() as u64 {
                return Err(Error::DataTooLarge(format!("File size (currently {} bytes) is too large for FBX {}", last_pos, self.version)));
            }
            try!(sink.write_u32::<LittleEndian>(last_pos as u32));
        } else {
            try!(sink.write_u64::<LittleEndian>(last_pos));
        }
        try!(sink.seek(SeekFrom::Start(last_pos)));

        Ok(())
    }
}
