use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::FbxEvent;
use super::CommonState;

#[derive(Debug, Clone)]
pub struct BinaryParser {
    version: u32,
}

impl BinaryParser {
    pub fn new(version: u32) -> Self {
        BinaryParser {
            version: version,
        }
    }

    pub fn next<R: Read>(&mut self, reader: &mut R, common: &mut CommonState) -> Result<FbxEvent> {
        let node_record_header = try!(NodeRecordHeader::load(reader, &mut common.pos));
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
    pub fn load<R: Read>(_reader: &mut R, pos: &mut u64) -> Result<Self> {
        Err(Error::new(*pos, ErrorKind::Unimplemented("Parser for NodeRecordHeader is not implemented yet".to_string())))
    }

    /// Check whether the header indicates there are no more children.
    pub fn is_null_record(&self) -> bool {
        self.end_offset == 0
            && self.num_properties == 0
            && self.property_list_len == 0
            && self.name_len == 0
    }
}
