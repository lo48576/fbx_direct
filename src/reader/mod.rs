use std::io::Read;
use error::Result;

mod parser;

#[derive(Debug, Clone, Copy)]
pub enum FbxFormatType {
    Binary(u32),
    Text,
}

#[derive(Debug, Clone)]
pub enum FbxEvent {
    StartFbx(FbxFormatType),
    EndFbx,
    StartNode {
        name: String,
        properties: Vec<PropertyValue>,
    },
    EndNode,
    Comment(String),
}

pub struct EventReader<R: Read> {
    source: R,
    parser: parser::Parser,
}

impl<R: Read> EventReader<R> {
    pub fn new(source: R) -> Self {
        EventReader {
            source: source,
            parser: parser::Parser::new(),
        }
    }

    pub fn next(&mut self) -> Result<FbxEvent> {
        self.parser.next(&mut self.source)
    }
}

impl <R: Read> IntoIterator for EventReader<R> {
    type Item = Result<FbxEvent>;
    type IntoIter = Events<R>;

    fn into_iter(self) -> Events<R> {
        Events {
            reader: self,
            finished: false,
        }
    }
}

/// An iterator over FBX events created from some type implementing `Read`.
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
    /// 4 byte single-precision IEEE 754 number.
    F32(f32),
    /// 8 byte double-precision IEEE 754 number.
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
    String(String),
    /// Raw binary data.
    Binary(Vec<u8>),
}
