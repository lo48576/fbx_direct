//! Contains FBX emitters.

use self::ascii::AsciiEmitter;
use self::binary::BinaryEmitter;
use crate::common::FbxFormatType;
use crate::writer::error::{Error, Result};
use crate::writer::{EmitterConfig, FbxEvent};
use log::{error, warn};
use std::io::{Seek, Write};

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
struct CommonState {
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
            config,
            common: CommonState { final_result: None },
            state: EmitterState::Initial,
        }
    }

    pub fn write<'a, W: Write + Seek>(&mut self, sink: &mut W, event: FbxEvent<'a>) -> Result<()> {
        if let Some(ref result) = self.common.final_result {
            return result.clone();
        }
        let result = match self.state {
            EmitterState::Initial => match event {
                FbxEvent::StartFbx(FbxFormatType::Binary(ver)) => {
                    if let Some(config_fbx_ver) = self.config.fbx_version {
                        if ver != config_fbx_ver {
                            return Err(Error::InvalidOption(format!("FBX version {} specified by emitter config, but {} is given for `StartFbx` event", config_fbx_ver, ver)));
                        }
                    }
                    let mut emitter = BinaryEmitter::new(ver);
                    let result = emitter.emit_start_fbx(sink, ver);
                    self.state = EmitterState::Binary(emitter);
                    result
                }
                FbxEvent::StartFbx(FbxFormatType::Ascii) => {
                    let mut emitter = AsciiEmitter::new();
                    let result = if let Some(ver) = self.config.fbx_version {
                        emitter.emit_start_fbx(sink, ver)
                    } else {
                        Err(Error::InvalidOption(
                            "Attempt to export ASCII FBX but version is not specified".to_string(),
                        ))
                    };
                    self.state = EmitterState::Ascii(emitter);
                    result
                }
                _ => Err(Error::FbxNotStarted),
            },
            EmitterState::Binary(ref mut emitter) => match event {
                FbxEvent::StartFbx(_) => Err(Error::FbxAlreadyStarted),
                FbxEvent::EndFbx => emitter.emit_end_fbx(sink),
                FbxEvent::StartNode { name, properties } => {
                    emitter.emit_start_node(sink, name, &properties)
                }
                FbxEvent::EndNode => emitter.emit_end_node(sink),
                FbxEvent::Comment(_) => {
                    if self.config.ignore_minor_errors {
                        warn!("Comment cannot be exported to Binary FBX");
                        Ok(())
                    } else {
                        error!("Comment cannot be exported to Binary FBX");
                        Err(Error::UnwritableEvent)
                    }
                }
            },
            EmitterState::Ascii(ref mut emitter) => match event {
                FbxEvent::StartFbx(_) => Err(Error::FbxAlreadyStarted),
                FbxEvent::EndFbx => emitter.emit_end_fbx(sink),
                FbxEvent::StartNode { name, properties } => {
                    emitter.emit_start_node(sink, name, &properties)
                }
                FbxEvent::EndNode => emitter.emit_end_node(sink),
                FbxEvent::Comment(comment) => emitter.emit_comment(sink, comment),
            },
        };
        if let Err(ref err) = result {
            self.common.final_result = Some(Err(err.clone()));
        }
        result
    }
}
