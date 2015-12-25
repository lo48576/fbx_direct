use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::FbxEvent;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParserMode {
    /// Reading magic binary (i.e. the first line)
    Magic,
    /// Reading binary FBX
    Binary,
    /// Reading ASCII FBX
    Ascii,
}

pub struct Parser {
    pos: u64,
    mode: ParserMode,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            pos: 0,
            mode: ParserMode::Magic,
        }
    }
    pub fn next<R: Read>(&mut self, reader: &mut R) -> Result<FbxEvent> {
        match self.mode {
            ParserMode::Magic => self.magic_next(reader),
            ParserMode::Binary => self.binary_next(reader),
            ParserMode::Ascii => self.ascii_next(reader),
        }
    }
    pub fn magic_next<R: Read>(&mut self, _reader: &mut R) -> Result<FbxEvent> {
        Err(Error::new(self.pos, ErrorKind::Unimplemented("Parser for magic binary is not implemented yet".to_string())))
    }
    pub fn binary_next<R: Read>(&mut self, _reader: &mut R) -> Result<FbxEvent> {
        Err(Error::new(self.pos, ErrorKind::Unimplemented("Parser for Binary FBX format is not implemented yet".to_string())))
    }
    pub fn ascii_next<R: Read>(&mut self, _reader: &mut R) -> Result<FbxEvent> {
        Err(Error::new(self.pos, ErrorKind::Unimplemented("Parser for ASCII FBX format is not implemented yet".to_string())))
    }
}
