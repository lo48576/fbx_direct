//! Contains result and error type for FBX reader.

extern crate byteorder;

use std::io;
use std::fmt;
use std::error;

/// A specialized `std::result::Result` type for FBX exporting.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An FBX parsing error.
#[derive(Debug)]
pub enum Error {
    /// I/O error.
    Io(io::Error),
    /// `EndNode` event is given but there's no node to close.
    ExtraEndNode,
    /// FBX not started but an event other than `StartFbx` is given.
    FbxNotStarted,
    /// FBX is already started but `StartFbx` is given.
    FbxAlreadyStarted,
    /// Unsupported FBX version.
    UnsupportedFbxVersion,
    /// Given event is not writable in current format.
    UnwritableEvent,
    /// Unimplemented feature.
    Unimplemented(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
            Error::ExtraEndNode => write!(f, "Extra end-of-node marker detected"),
            Error::FbxNotStarted => write!(f, "An writer event is given, but FBX data is not started yet"),
            Error::FbxAlreadyStarted => write!(f, "Got a writer event to start FBX, but FBX data is already started"),
            Error::UnsupportedFbxVersion => write!(f, "Unsupported FBX version"),
            Error::UnwritableEvent => write!(f, "A given event is not writable in current format"),
            Error::Unimplemented(ref err) => write!(f, "Unimplemented feature: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::ExtraEndNode => "Extra end-of-node marker detected",
            Error::FbxNotStarted => "An writer event is given, but FBX data is not started yet",
            Error::FbxAlreadyStarted => "Got a writer event to start FBX, but FBX data is already started",
            Error::UnsupportedFbxVersion => "Unsupported FBX version",
            Error::UnwritableEvent => "A given event is not writable in current format",
            Error::Unimplemented(_) => "Attempt to use unimplemented feature",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        use self::Error::*;
        use std::error::Error;
        match *self {
            Io(ref e) => Io(io::Error::new(e.kind(), e.description())),
            Unimplemented(ref e) => Unimplemented(e.clone()),
            ref e => e.clone(),
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
