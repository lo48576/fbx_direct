//! Contains implementation of Binary FBX emitter.

use std::io::{Write, Seek};
use std::borrow::Cow;
use writer::error::{Result, Error};
use common::Property;

/// A writer for Binary FBX.
#[derive(Debug, Clone)]
pub struct BinaryEmitter {
    version: u32,
    pos: u64,
    end_offset_pos_stack: Vec<u64>,
}

impl BinaryEmitter {
    /// Constructs Binary FBX writer with FBX version.
    pub fn new(version: u32) -> Self {
        BinaryEmitter {
            version: version,
            pos: 0,
            end_offset_pos_stack: vec![],
        }
    }

    pub fn emit_start_fbx<W: Write + Seek>(&mut self, sink: &mut W, ver: u32) -> Result<()> {
        Err(Error::Unimplemented("BinaryEmitter::emit_start_fbx() is unimplemented yet".to_string()))
    }

    pub fn emit_end_fbx<W: Write + Seek>(&mut self, sink: &mut W) -> Result<()> {
        Err(Error::Unimplemented("BinaryEmitter::emit_end_fbx() is unimplemented yet".to_string()))
    }

    pub fn emit_start_node<W: Write + Seek>(&mut self, sink: &mut W, name: &str, properties: &[Property]) -> Result<()> {
        Err(Error::Unimplemented("BinaryEmitter::emit_start_node() is unimplemented yet".to_string()))
    }

    pub fn emit_end_node<W: Write + Seek>(&mut self, sink: &mut W) -> Result<()> {
        Err(Error::Unimplemented("BinaryEmitter::emit_end_node() is unimplemented yet".to_string()))
    }
}
