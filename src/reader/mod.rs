//! Contains high-level interface for a pull-based (StAX-like) FBX parser.

use std::io::Read;
use self::error::Result;

pub use self::error::{Error, ErrorKind};
use common::{FbxFormatType, OwnedProperty};

mod error;
mod parser;

/// A node of an FBX input stream.
///
/// Items of this enum are emitted by [`reader::EventReader`](struct.EventReader.html).
#[derive(Debug, Clone)]
pub enum FbxEvent {
    /// Denotes start of FBX data.
    ///
    /// For Binary FBX, this item corresponds to magic binary.
    StartFbx(FbxFormatType),
    /// Denotes end of FBX data.
    ///
    /// NOTE: Current implementation of Binary FBX parser does not read to the last byte of the FBX stream.
    EndFbx,
    /// Denotes beginning of a node.
    StartNode {
        /// Node name.
        name: String,
        /// Node properties.
        properties: Vec<OwnedProperty>,
    },
    /// Denotes end of a node.
    EndNode,
    /// Comment.
    ///
    /// Comment only appears in ASCII FBX.
    Comment(String),
}

/// A wrapper around an `std::io::Read` instance which provides pull-based FBX parsing.
pub struct EventReader<R: Read> {
    source: R,
    parser: parser::Parser,
}

impl<R: Read> EventReader<R> {
    /// Creates a new reader, consuming the given stream.
    pub fn new(source: R) -> Self {
        EventReader {
            source: source,
            parser: parser::Parser::new(ParserConfig::new()),
        }
    }

    /// Creates a new reader with provided configuration, consuming the given stream.
    pub fn new_with_config(source: R, config: ParserConfig) -> Self {
        EventReader {
            source: source,
            parser: parser::Parser::new(config),
        }
    }

    /// Pulls and returns next FBX event from the stream.
    pub fn next(&mut self) -> Result<FbxEvent> {
        self.parser.next(&mut self.source)
    }
}

impl <R: Read> IntoIterator for EventReader<R> {
    type Item = Result<FbxEvent>;
    type IntoIter = Events<R>;

    /// Consumes `EventReader` and returns an iterator (`Events`) over it.
    fn into_iter(self) -> Events<R> {
        Events {
            reader: self,
            finished: false,
        }
    }
}

/// An iterator over FBX events created from some type implementing `Read`.
///
/// When the next event is [`reader::error::Error`](struct.Error.html) or
/// [`reader::FbxEvent::EndFbx`](enum.FbxEvent.html) then it will be returned
/// by the iterator once, and then it will stop producing events.
pub struct Events<R: Read> {
    reader: EventReader<R>,
    finished: bool,
}

impl<R: Read> Events<R> {
    /// Returns internal `EventReader`.
    #[allow(dead_code)]
    fn into_inner(self) -> EventReader<R> {
        self.reader
    }
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<FbxEvent>;

    fn next(&mut self) -> Option<Result<FbxEvent>> {
        if self.finished {
            None
        } else {
            let ev = self.reader.next();
            match ev {
                Ok(FbxEvent::EndFbx) | Err(_) => self.finished = true,
                _ => {}
            }
            Some(ev)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserConfig {
    pub ignore_comments: bool,
}

impl ParserConfig {
    /// Creates a new config with default options.
    pub fn new() -> Self {
        ParserConfig {
            ignore_comments: false,
        }
    }

    /// Creates an FBX reader with this configuration.
    pub fn create_reader<R: Read>(self, source: R) -> EventReader<R> {
        EventReader::new_with_config(source, self)
    }

    /// Sets the field to provided value and returns updated config object.
    pub fn ignore_comments(mut self, value: bool) -> Self {
        self.ignore_comments = value;
        self
    }
}

impl Default for ParserConfig {
    fn default() -> ParserConfig {
        ParserConfig::new()
    }
}
