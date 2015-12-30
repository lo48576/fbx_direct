//! Contains implementation of Binary FBX emitter.

extern crate byteorder;

use std::io::{Write, Seek};
use std::borrow::Cow;
use std::io::SeekFrom;
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
        if ver < 7000 {
            error!("Unsupported version: {}", ver);
            return Err(Error::UnsupportedFbxVersion);
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
        Err(Error::Unimplemented("BinaryEmitter::emit_end_fbx() is unimplemented yet".to_string()))
    }

    pub fn emit_start_node<W: Write + Seek>(&mut self, sink: &mut W, name: &str, properties: &[Property]) -> Result<()> {
        if let Some(top) = self.null_record_necessities.last_mut() {
            // Parent node requires null record, because it has child node (the current node!).
            *top = true;
        }
        self.null_record_necessities.push(properties.is_empty());

        // Write node record header.
        // For detail of node record header, see `reader::parser::binary::NodeRecordHeader` struct.
        // Write a placeholder for `end_offset` and remember current offset.
        self.end_offset_pos_stack.push(try!(sink.seek(SeekFrom::Current(0))));
        try!(sink.write_u32::<LittleEndian>(0xef_be_ad_de));
        // Write `num_properties`.
        try!(sink.write_u32::<LittleEndian>(properties.len() as u32));
        // Write a default value of `property_list_len`.
        let prop_list_len_offset = try!(sink.seek(SeekFrom::Current(0)));
        try!(sink.write_u32::<LittleEndian>(0));
        // Write length of the node name.
        try!(sink.write_u8(name.len() as u8));

        // Write a node name.
        try!(sink.write_all(name.as_bytes()));

        // Write properties.
        if !properties.is_empty() {
            let mut props_byte_size = 0_u32;
            for prop in properties {
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
                        vec.len()
                    },
                    Property::String(s) => {
                        try!(sink.write_u8('S' as u8));
                        try!(sink.write_u32::<LittleEndian>(s.len() as u32));
                        try!(sink.write_all(s.as_bytes()));
                        4 + s.len()
                    },
                    Property::Binary(b) => {
                        try!(sink.write_u8('R' as u8));
                        try!(sink.write_u32::<LittleEndian>(b.len() as u32));
                        try!(sink.write_all(b));
                        4 + b.len()
                    },
                    _ => {
                        return Err(Error::Unimplemented("BinaryEmitter::emit_start_node() is unimplemented yet".to_string()));
                    }
                } as u32;
            }
            // Update `property_list_len`
            let last_pos = try!(sink.seek(SeekFrom::Current(0)));
            try!(sink.seek(SeekFrom::Start(prop_list_len_offset)));
            try!(sink.write_u32::<LittleEndian>(props_byte_size));
            try!(sink.seek(SeekFrom::Start(last_pos)));
        }

        Ok(())
    }

    pub fn emit_end_node<W: Write + Seek>(&mut self, sink: &mut W) -> Result<()> {
        // Write a null record header if necessary.
        if let Some(required) = self.null_record_necessities.pop() {
            if required {
                // 13: size of a node record header.
                try!(sink.write_all(&[0; 13]));
            }
        } else {
            return Err(Error::ExtraEndNode);
        }

        // Update `end_offset`.
        let last_pos = try!(sink.seek(SeekFrom::Current(0)));
        try!(sink.seek(SeekFrom::Start(self.end_offset_pos_stack.pop().unwrap())));
        try!(sink.write_u32::<LittleEndian>(last_pos as u32));
        try!(sink.seek(SeekFrom::Start(last_pos)));

        Ok(())
    }
}
