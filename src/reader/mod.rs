//! Contains high-level interface for a pull-based (StAX-like) FBX parser.

use std::io::Read;
use self::error::Result;

pub use self::error::{Error, ErrorKind};

mod error;
mod parser;

/// Format of FBX data
#[derive(Debug, Clone, Copy)]
pub enum FbxFormatType {
    /// Binary FBX, with version (for example, 7400 for FBX 7.4).
    Binary(u32),
    /// ASCII FBX.
    Text,
}

/// A node of an FBX input stream.
///
/// Items of this enum are emitted by `reader::EventReader`.
#[derive(Debug, Clone)]
pub enum FbxEvent {
    /// Denotes start of FBX data.
    ///
    /// For Binary FBX, this item corresponds to magic binary.
    StartFbx(FbxFormatType),
    /// Denotes end of FBX data.
    ///
    /// NOTE: Current implementation of `reader::parser::BinaryParser` does not read to end of the
    ///       FBX stream.
    EndFbx,
    /// Denotes beginning of a node.
    StartNode {
        /// Node name.
        name: String,
        /// Node properties.
        properties: Vec<PropertyValue>,
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
            parser: parser::Parser::new(),
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
/// When the next event is `reader::error::Error` or `reader::FbxEvent::EndFbx`, then it will be
/// returned by the iterator once, and then it will stop producing events.
pub struct Events<R: Read> {
    reader: EventReader<R>,
    finished: bool,
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

/// A property type of the FBX node.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Boolean.
    Bool(bool),
    /// 2 byte signed integer.
    I16(i16),
    /// 4 byte signed integer.
    I32(i32),
    /// 8 byte signed integer.
    I64(i64),
    /// 4 byte single-precision IEEE 754 floating-point number.
    F32(f32),
    /// 8 byte double-precision IEEE 754 floating-point number.
    F64(f64),
    /// Array of boolean.
    VecBool(Vec<bool>),
    /// Array of 4 byte signed integer.
    VecI32(Vec<i32>),
    /// Array of 8 byte signed integer.
    VecI64(Vec<i64>),
    /// Array of 4 byte single-precision IEEE 754 number.
    VecF32(Vec<f32>),
    /// Array of 8 byte double-precision IEEE 754 number.
    VecF64(Vec<f64>),
    /// String.
    ///
    /// Note that the string can contain special character like `\u{0}`.
    String(String),
    /// Raw binary data.
    Binary(Vec<u8>),
}
