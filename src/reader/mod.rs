use std::io::Read;
use ::Result;

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
