//! Contains implementation of Binary FBX emitter.

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
}
