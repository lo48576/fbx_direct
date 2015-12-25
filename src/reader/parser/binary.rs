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

    pub fn next<R: Read>(&mut self, _reader: &mut R, common: &mut CommonState) -> Result<FbxEvent> {
        Err(Error::new(common.pos, ErrorKind::Unimplemented("Parser for Binary FBX format is not implemented yet".to_string())))
    }
}
