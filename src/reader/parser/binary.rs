//! Contains implementation of Binary FBX parser.

extern crate byteorder;
extern crate flate2;

use std::io::Read;
use reader::error::{Result, Error, ErrorKind};
use reader::{FbxEvent, PropertyValue};
use super::CommonState;

/// A parser for Binary FBX.
#[derive(Debug, Clone)]
pub struct BinaryParser {
    version: u32,
    end_offset_stack: Vec<u32>,
}

impl BinaryParser {
    /// Constructs Binary FBX parser with FBX version (which is placed after magic binary).
    pub fn new(version: u32) -> Self {
        BinaryParser {
            version: version,
            end_offset_stack: vec![],
        }
    }

    pub fn next<R: Read>(&mut self, reader: &mut R, common: &mut CommonState) -> Result<FbxEvent> {
        // Check if the previously read node ends here.
        if let Some(&end_pos_top) = self.end_offset_stack.last() {
            if end_pos_top as u64 == common.pos {
                // Reached the end of previously read node.
                self.end_offset_stack.pop();
                return Ok(FbxEvent::EndNode);
            }
        }

        // Read a node record header.
        let node_record_header = try!(NodeRecordHeader::read(reader, &mut common.pos));
        if node_record_header.is_null_record() {
            // End of a node.
            return if let Some(expected_pos) = self.end_offset_stack.pop() {
                if common.pos == expected_pos as u64 {
                    Ok(FbxEvent::EndNode)
                } else {
                    // Data is collapsed (the node doesn't end at expected position).
                    Err(Error::new(
                            common.pos,
                            ErrorKind::DataError(format!("Node does not end at expected position (expected {}, now at {})", expected_pos, common.pos))))
                }
            } else {
                // Reached end of all nodes.
                // (Extra NULL-record header is end marker of implicit root node.)
                // Footer with unknown contents follows.
                // TODO: Read footer.
                //       Files exported by official products or SDK have padding and their file
                //       sizes are multiple of 16, but some files exported by third-party apps
                //       (such as blender) does not.
                //       So it may be difficult to check if the footer is correct or wrong.
                // NOTE: There is the only thing known, the last 16 bytes of the data always seem
                //       to be `[0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c,
                //       0xe3, 0x75, 0x8f, 0x29, 0x0b]`.
                Ok(FbxEvent::EndFbx)
            };
        } else {
            // Start of a node.
            self.end_offset_stack.push(node_record_header.end_offset);
        }

        // Read a node name.
        let name = try_read_fixstr!(common.pos, reader, node_record_header.name_len);

        // Read properties.
        let mut properties = Vec::<PropertyValue>::with_capacity(node_record_header.num_properties as usize);
        for _ in 0..node_record_header.num_properties {
            let prop = try!(self.read_property(reader, common));
            properties.push(prop);
        }

        Ok(FbxEvent::StartNode {
            name: name,
            properties: properties,
        })
    }

    /// Read a node property value.
    fn read_property<R: Read>(&mut self, reader: &mut R, common: &mut CommonState) -> Result<PropertyValue> {
        let type_code = try_read_le_u8!(common.pos, reader);
        // type code must be ASCII.
        let type_code = if type_code > 0x80 {
            return Err(Error::new(common.pos-1, ErrorKind::DataError(format!("Expected property type code (ASCII) but got {:#x}", type_code))));
        } else {
            type_code as char
        };
        let value = match type_code {
            // 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 byte value.
            'C' => {
                let val = try_read_le_u8!(common.pos, reader);
                // It seems 'T' (0x54) is used as `false`, 'T' (0x59) is used as `true`.
                if (val != 'T' as u8) && (val != 'Y' as u8) {
                    // Should this treated as error?
                    // (I don't know whether other characters than 'T' and 'Y' are allowed...)
                    warn!("Expected 'T' or 'Y' for representaton of boolean property value, but got {:#x}", val);
                }
                // Check LSB.
                PropertyValue::Bool(val & 1 == 1)
            },
            // 2 byte signed integer.
            'Y' => {
                PropertyValue::I16(try_read_le_i16!(common.pos, reader))
            },
            // 4 byte signed integer.
            'I' => {
                PropertyValue::I32(try_read_le_i32!(common.pos, reader))
            },
            // 4 byte single-precision IEEE 754 floating-point number.
            'F' => {
                PropertyValue::F32(try_read_le_f32!(common.pos, reader))
            },
            // 8 byte double-precision IEEE 754 floating-point number.
            'D' => {
                PropertyValue::F64(try_read_le_f64!(common.pos, reader))
            },
            // 8 byte signed integer.
            'L' => {
                PropertyValue::I64(try_read_le_i64!(common.pos, reader))
            },
            // Array types
            'f'|'d'|'l'|'i'|'b' => {
                let array_header = try!(PropertyArrayHeader::read(reader, &mut common.pos));
                try!(self.read_property_value_array(reader, common, type_code, &array_header))
            },
            // String
            'S' => {
                let length = try_read_le_u32!(common.pos, reader);
                PropertyValue::String(try_read_fixstr!(common.pos, reader, length))
            },
            // Raw binary data
            'R' => {
                let length = try_read_le_u32!(common.pos, reader);
                PropertyValue::Binary(try_read_exact!(common.pos, reader, length))
            },
            _ => {
                return Err(Error::new(
                        common.pos,
                        ErrorKind::UnexpectedValue(format!(
                                "Unsupported type code appears in node property: type_code={}({:#x})",
                                type_code, type_code as u8))));
            }
        };
        Ok(value)
    }

    /// Read a property value of array type from given stream which maybe compressed.
    fn read_property_value_array<R: Read>(&mut self,
                                          reader: &mut R, common: &mut CommonState,
                                          type_code: char, array_header: &PropertyArrayHeader) -> Result<PropertyValue> {
        match array_header.encoding {
            // 0; raw
            0 => {
                let (val, byte_size) = try!(self.read_property_value_array_from_plain_stream(reader, common.pos, type_code, array_header.array_length));
                common.pos += byte_size;
                Ok(val)
            },
            // 1: zlib compressed data
            1 => {
                let mut decoded_stream = flate2::read::ZlibDecoder::new(reader.by_ref().take(array_header.compressed_length as u64));
                let (val, _) = try!(self.read_property_value_array_from_plain_stream(&mut decoded_stream, common.pos, type_code, array_header.array_length));
                common.pos += array_header.compressed_length as u64;
                Ok(val)
            },
            // Unknown.
            e => {
                Err(Error::new(
                        common.pos,
                        ErrorKind::UnexpectedValue(format!("Unsupported property array encoding, got {:#x}", e))))
            }
        }
    }

    /// Read a property value of array type from plain (uncompressed) stream.
    fn read_property_value_array_from_plain_stream<R: Read>(&mut self, reader: &mut R, abs_pos: u64, type_code: char,
                                                            num_elements: u32) -> Result<(PropertyValue, u64)> {
        use self::byteorder::{ReadBytesExt, LittleEndian};
        Ok(match type_code {
            // Array of 4 byte single-precision IEEE 754 floating-point number.
            'f' => {
                let mut data = Vec::<f32>::with_capacity(num_elements as usize);
                for _ in 0..num_elements {
                    data.push(try_with_pos!(abs_pos, reader.read_f32::<LittleEndian>()));
                }
                (PropertyValue::VecF32(data), num_elements as u64 * 4)
            },
            // Array of 8 byte double-precision IEEE 754 floating-point number.
            'd' => {
                let mut data = Vec::<f64>::with_capacity(num_elements as usize);
                for _ in 0..num_elements {
                    data.push(try_with_pos!(abs_pos, reader.read_f64::<LittleEndian>()));
                }
                (PropertyValue::VecF64(data), num_elements as u64 * 8)
            },
            // Array of 8 byte signed integer.
            'l' => {
                let mut data = Vec::<i64>::with_capacity(num_elements as usize);
                for _ in 0..num_elements {
                    data.push(try_with_pos!(abs_pos, reader.read_i64::<LittleEndian>()));
                }
                (PropertyValue::VecI64(data), num_elements as u64 * 8)
            },
            // Array of 4 byte signed integer.
            'i' => {
                let mut data = Vec::<i32>::with_capacity(num_elements as usize);
                for _ in 0..num_elements {
                    data.push(try_with_pos!(abs_pos, reader.read_i32::<LittleEndian>()));
                }
                (PropertyValue::VecI32(data), num_elements as u64 * 4)
            },
            // Array of 1 byte booleans (always 0 or 1?).
            'b' => {
                let mut data = Vec::<bool>::with_capacity(num_elements as usize);
                for _ in 0..num_elements {
                    // Check LSB.
                    data.push(try_with_pos!(abs_pos, reader.read_u8()) & 1 == 1);
                }
                (PropertyValue::VecBool(data), num_elements as u64)
            },
            _ => {
                // Unreachable because `read_property()` gives only 'f' , 'd', 'l', 'i', or 'b' to
                // `read_property_value_array()`.
                unreachable!();
            }
        })
    }
}

/// A header of a node.
#[derive(Debug, Copy, Clone)]
struct NodeRecordHeader {
    /// Position of the end of the node.
    end_offset: u32,
    /// Number of the properties the node has.
    num_properties: u32,
    /// Byte size of properties of the node in the FBX stream.
    property_list_len: u32,
    /// Byte size of the node name.
    name_len: u8,
}

impl NodeRecordHeader {
    /// Constructs `NodeRecordHeader` from the given stream.
    pub fn read<R: Read>(reader: &mut R, pos: &mut u64) -> Result<Self> {
        let end_offset = try_read_le_u32!(*pos, reader);
        let num_properties = try_read_le_u32!(*pos, reader);
        let property_list_len = try_read_le_u32!(*pos, reader);
        let name_len = try_read_le_u8!(*pos, reader);
        Ok(NodeRecordHeader {
            end_offset: end_offset,
            num_properties: num_properties,
            property_list_len: property_list_len,
            name_len: name_len,
        })
    }

    /// Check whether the header indicates there are no more children.
    pub fn is_null_record(&self) -> bool {
        self.end_offset == 0
            && self.num_properties == 0
            && self.property_list_len == 0
            && self.name_len == 0
    }
}

/// A header of a property of array type.
#[derive(Debug, Copy, Clone)]
pub struct PropertyArrayHeader {
    /// Number of values in the array, *NOT byte size*.
    array_length: u32,
    /// Denotes whether data in stream is plain, or what algorithm it is compressed by.
    encoding: u32,
    /// Byte size of the compressed array value in the stream.
    compressed_length: u32,
}

impl PropertyArrayHeader {
    /// Constructs `PropertyArrayHeader` from the given stream.
    pub fn read<R: Read>(reader: &mut R, pos: &mut u64) -> Result<Self> {
        let array_length = try_read_le_u32!(*pos, reader);
        let encoding = try_read_le_u32!(*pos, reader);
        let compressed_length = try_read_le_u32!(*pos, reader);
        Ok(PropertyArrayHeader {
            array_length: array_length,
            encoding: encoding,
            compressed_length: compressed_length,
        })
    }
}
