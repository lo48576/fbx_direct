use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::{FbxEvent, PropertyValue};
use super::CommonState;

#[derive(Debug, Clone)]
pub struct BinaryParser {
    version: u32,
    end_offset_stack: Vec<u32>,
}

impl BinaryParser {
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
        let node_record_header = try!(NodeRecordHeader::load(reader, &mut common.pos));
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
                // Data is collapsed (an extra node end marker found).
                Err(Error::new(common.pos, ErrorKind::DataError("An extra node end marker found".to_string())))
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
                return Err(Error::new(
                        common.pos,
                        ErrorKind::Unimplemented("Parser for array type of property value is not implemented yet".to_string())));
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
                        ErrorKind::DataError(format!(
                                "Unsupported type code appears in node property: type_code={}({:#x})",
                                type_code, type_code as u8))));
            }
        };
        Ok(value)
    }
}

#[derive(Debug, Copy, Clone)]
struct NodeRecordHeader {
    end_offset: u32,
    num_properties: u32,
    property_list_len: u32,
    name_len: u8,
}

impl NodeRecordHeader {
    pub fn load<R: Read>(reader: &mut R, pos: &mut u64) -> Result<Self> {
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
