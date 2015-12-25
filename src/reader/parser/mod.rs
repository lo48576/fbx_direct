use std::io::Read;
use error::{Error, ErrorKind};
use ::Result;
use reader::FbxEvent;

pub struct Parser {
    pos: u64,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            pos: 0,
        }
    }
    pub fn next<R: Read>(&mut self, _reader: &mut R) -> Result<FbxEvent> {
        Err(Error::new(self.pos, ErrorKind::Unimplemented("Parser is not implemented yet".to_string())))
    }
}
