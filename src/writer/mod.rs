//! Contains interface for an events-based FBX emitter.

use std::io::{Write, Seek};

pub use self::error::{Result, Error};
pub use self::events::FbxEvent;

mod emitter;
mod error;
mod events;

/// A wrapper around an `std::io::Write` instance which emits Binary FBX.
pub struct EventWriter<W: Write + Seek> {
    sink: W,
    emitter: emitter::Emitter,
}

impl<W: Write + Seek> EventWriter<W> {
    /// Creates a new writer.
    pub fn new(sink: W) -> Self {
        EventWriter {
            sink: sink,
            emitter: emitter::Emitter::new(EmitterConfig::new()),
        }
    }

    /// Creates a new emitter with provided configuration.
    pub fn new_with_config(sink: W, config: EmitterConfig) -> Self {
        EventWriter {
            sink: sink,
            emitter: emitter::Emitter::new(config),
        }
    }

    /// Writes the next piece of FBX fragment according to the provided event.
    pub fn write<'a, E>(&mut self, event: E) -> Result<()>
        where E: Into<FbxEvent<'a>>
    {
        self.emitter.write(&mut self.sink, event.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmitterConfig {
    pub ignore_minor_errors: bool,
    pub fbx_version: Option<u32>,
}

impl EmitterConfig {
    /// Creates a new config with default options.
    pub fn new() -> Self {
        EmitterConfig {
            ignore_minor_errors: true,
            fbx_version: None,
        }
    }

    /// Creates an FBX writer with this configuration.
    pub fn create_writer<W: Write + Seek>(self, sink: W) -> EventWriter<W> {
        EventWriter::new_with_config(sink, self)
    }

    /// Sets the field to provided value and returns updated config object.
    pub fn ignore_minor_errors(mut self, value: bool) -> Self {
        self.ignore_minor_errors = value;
        self
    }

    /// Sets the FBX version to write.
    pub fn fbx_version(mut self, value: Option<u32>) -> Self {
        self.fbx_version = value;
        self
    }
}

impl Default for EmitterConfig {
    fn default() -> EmitterConfig {
        EmitterConfig::new()
    }
}
