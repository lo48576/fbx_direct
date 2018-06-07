//! Contains implementation of ASCII FBX parser.

use std::io::Read;
use reader::error::{Result, Error, ErrorKind};
use reader::FbxEvent;
use super::CommonState;

/// A parser for ASCII FBX.
#[derive(Debug, Clone)]
pub struct AsciiParser {
    buffer: String,
}

impl AsciiParser {
    /// Constructs ASCII FBX parser with initial state of internal buffer.
    pub(crate) fn new(buffer: String) -> Self {
        AsciiParser {
            buffer: buffer,
        }
    }

    pub(crate) fn next<R: Read>(&mut self, _reader: &mut R, common: &mut CommonState) -> Result<FbxEvent> {
        Err(Error::new(common.pos, ErrorKind::Unimplemented("Parser for ASCII FBX format is not implemented yet".to_string())))
    }
}
