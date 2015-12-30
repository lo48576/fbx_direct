//! Contains FBX emitters.

use std::io::Write;
use writer::error::{Result, Error};
use writer::{FbxEvent, EmitterConfig};
use common::FbxFormatType;

pub struct Emitter {
    config: EmitterConfig,
}

impl Emitter {
    pub fn new(config: EmitterConfig) -> Self {
        Emitter {
            config: config,
        }
    }
}
