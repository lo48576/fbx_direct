use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::FbxEvent;
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
        // Read a node record header.
        let node_record_header = try!(NodeRecordHeader::load(reader, &mut common.pos));
        self.end_offset_stack.push(node_record_header.end_offset);

        // Read properties.
        Err(Error::new(common.pos, ErrorKind::Unimplemented("Parser for Binary FBX format is not implemented yet".to_string())))
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
