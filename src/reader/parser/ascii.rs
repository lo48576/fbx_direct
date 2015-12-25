use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::FbxEvent;
use super::CommonState;

#[derive(Debug, Clone)]
pub struct AsciiParser {
    buffer: String,
}

impl AsciiParser {
    pub fn new(buffer: String) -> Self {
        AsciiParser {
            buffer: buffer,
        }
    }

    pub fn next<R: Read>(&mut self, _reader: &mut R, common: &mut CommonState) -> Result<FbxEvent> {
        Err(Error::new(common.pos, ErrorKind::Unimplemented("Parser for ASCII FBX format is not implemented yet".to_string())))
    }
}
