//! Contains implementation of ASCII FBX emitter.

use std::io::{Write, Seek};
use writer::error::{Result, Error};

/// A writer for ASCII FBX.
#[derive(Debug, Clone)]
pub struct AsciiEmitter;

impl AsciiEmitter {
    /// Constructs ASCII FBX writer.
    pub fn new() -> Self {
        AsciiEmitter
    }

    pub fn emit_start_fbx<W: Write + Seek>(&mut self, _sink: &mut W) -> Result<()> {
        Err(Error::Unimplemented("Ascii FBX emitter is unimplemented".to_string()))
    }
}
