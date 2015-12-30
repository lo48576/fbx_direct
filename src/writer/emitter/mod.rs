//! Contains FBX emitters.

use std::io::Write;
use writer::error::{Result, Error};
use writer::{FbxEvent, EmitterConfig};
use common::FbxFormatType;
use self::binary::BinaryEmitter;
use self::ascii::AsciiEmitter;

mod ascii;
mod binary;

#[derive(Debug, Clone)]
enum EmitterState {
    /// Emitter is initialized but not used yet.
    Initial,
    /// Emitting Binary FBX.
    Binary(BinaryEmitter),
    /// Emitting ASCII FBX.
    Ascii(AsciiEmitter),
}

#[derive(Debug, Clone)]
struct CommonState{
    final_result: Option<Result<()>>,
}

pub struct Emitter {
    config: EmitterConfig,
    common: CommonState,
    state: EmitterState,
}

impl Emitter {
    pub fn new(config: EmitterConfig) -> Self {
        Emitter {
            config: config,
            common: CommonState {
                final_result: Some(Ok(())),
            },
            state: EmitterState::Initial,
        }
    }

    pub fn write<'a>(&mut self, event: FbxEvent<'a>) -> Result<()> {
        Err(Error::Unimplemented("Emitter is unimplemented yet".to_string()))
    }
}
