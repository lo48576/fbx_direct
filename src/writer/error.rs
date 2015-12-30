//! Contains result and error type for FBX reader.

extern crate byteorder;

use std::io;

/// A specialized `std::result::Result` type for FBX exporting.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An FBX parsing error.
#[derive(Debug)]
pub enum Error {
    /// I/O error.
    Io(io::Error),
    /// FBX not started but an event other than `StartFbx` is given.
    FbxNotStarted,
    /// FBX is already started but `StartFbx` is given.
    FbxAlreadyStarted,
    /// Unimplemented feature.
    Unimplemented(String),
}

impl Clone for Error {
    fn clone(&self) -> Self {
        use self::Error::*;
        use std::error::Error;
        match *self {
            Io(ref e) => Io(io::Error::new(e.kind(), e.description())),
            FbxNotStarted => FbxNotStarted,
            FbxAlreadyStarted => FbxAlreadyStarted,
            Unimplemented(ref e) => Unimplemented(e.clone()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => panic!("byteorder::Error::UnexpectedEOF shouldn't happen on write"),
            byteorder::Error::Io(err) => Error::Io(err),
        }
    }
}
