extern crate byteorder;

use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::FbxEvent;

#[macro_use]
mod macros {
    macro_rules! try_with_pos {
        ($pos:expr, $expr:expr) => (match $expr {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => {
                return ::std::result::Result::Err($crate::error::Error::new($pos, err));
            },
        })
    }

    macro_rules! try_read_le_u32 {
        ($pos: expr, $reader:expr) => ({
            use self::byteorder::ReadBytesExt;
            let val = try_with_pos!($pos, $reader.by_ref().read_u32::<byteorder::LittleEndian>());
            $pos += 4;
            val
        })
    }

    macro_rules! try_read_exact {
        ($pos:expr, $reader:expr, $len:expr) => ({
            let mut buffer = Vec::<u8>::with_capacity($len as usize);
            let len = try_with_pos!($pos, $reader.by_ref().take($len as u64).read_to_end(&mut buffer)) as u64;
            if len != ($len as u64) {
                return Err(Error::new($pos, ErrorKind::UnexpectedEof));
            }
            $pos += len;
            buffer
        })
    }
}

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
