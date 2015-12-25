use std::io;
use std::string;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pos: u64,
    kind: ErrorKind,
}

impl Error {
    pub fn new<K: Into<ErrorKind>>(pos: u64, kind: K) -> Self {
        Error {
            pos: pos,
            kind: kind.into(),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    FromUtf8Error(string::FromUtf8Error),
    Io(io::Error),
    UnexpectedEof,
    Unimplemented(String),
}

impl From<string::FromUtf8Error> for ErrorKind {
    fn from(err: string::FromUtf8Error) -> ErrorKind {
        ErrorKind::FromUtf8Error(err)
    }
}

impl From<io::Error> for ErrorKind {
    fn from(err: io::Error) -> ErrorKind {
        ErrorKind::Io(err)
    }
}
