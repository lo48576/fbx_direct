//! Contains result and error type for FBX reader.

use std::error;
use std::fmt;
use std::io;
use std::str;
use std::string;

/// A specialized `std::result::Result` type for FBX parsing.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An FBX parsing error.
#[derive(Debug, Clone)]
pub struct Error {
    /// Last position of successfully read data when an error detected.
    pos: u64,
    /// Error type.
    kind: ErrorKind,
}

impl Error {
    /// Constructs `Error` with position and objects which can be converted to
    /// [`ErrorKind`](enum.ErrorKind.html).
    pub fn new<K: Into<ErrorKind>>(pos: u64, kind: K) -> Self {
        Error {
            pos: pos,
            kind: kind.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Utf8Error(ref err) => {
                write!(f, "UTF-8 conversion error at pos={}: {}", self.pos, err)
            }
            ErrorKind::InvalidMagic => write!(
                f,
                "Invalid magic header at pos={}: Non-FBX or corrupted data?",
                self.pos
            ),
            ErrorKind::Io(ref err) => write!(f, "I/O error at pos={}: {}", self.pos, err),
            ErrorKind::DataError(ref err) => write!(f, "Invalid data at pos={}: {}", self.pos, err),
            ErrorKind::UnexpectedValue(ref err) => {
                write!(f, "Got an unexpected value at pos={}: {}", self.pos, err)
            }
            ErrorKind::UnexpectedEof => write!(f, "Unexpected EOF at pos={}", self.pos),
            ErrorKind::Unimplemented(ref err) => write!(f, "Unimplemented feature: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Utf8Error(ref err) => err.description(),
            ErrorKind::InvalidMagic => "Got an invalid magic header",
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::DataError(_) => "Got an invalid data",
            ErrorKind::UnexpectedValue(_) => "Invalid value in FBX data",
            ErrorKind::UnexpectedEof => "Unexpected EOF",
            ErrorKind::Unimplemented(_) => "Attempt to use unimplemented feature",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self.kind {
            ErrorKind::Utf8Error(ref err) => Some(err as &dyn error::Error),
            ErrorKind::Io(ref err) => Some(err as &dyn error::Error),
            _ => None,
        }
    }
}

/// Error type.
#[derive(Debug)]
pub enum ErrorKind {
    /// Conversion from array of u8 to String failed.
    Utf8Error(str::Utf8Error),
    /// Invalid magic binary detected.
    InvalidMagic,
    /// I/O operation error.
    Io(io::Error),
    /// Corrupted or inconsistent FBX data detected.
    DataError(String),
    /// Got an unexpected value, and cannot continue parsing.
    ///
    /// This is specialization of [`DataError`](#variant.DataError).
    UnexpectedValue(String),
    /// Reached unexpected EOF.
    UnexpectedEof,
    /// Attempted to use unimplemented feature.
    Unimplemented(String),
}

impl Clone for ErrorKind {
    fn clone(&self) -> Self {
        use self::ErrorKind::*;
        use std::error::Error;
        match *self {
            Utf8Error(ref e) => Utf8Error(e.clone()),
            InvalidMagic => InvalidMagic,
            // `io::Error` (and an error wrapped by `io::Error`) cannot be cloned.
            Io(ref e) => Io(io::Error::new(e.kind(), e.description())),
            DataError(ref e) => DataError(e.clone()),
            UnexpectedValue(ref e) => UnexpectedValue(e.clone()),
            UnexpectedEof => UnexpectedEof,
            Unimplemented(ref e) => Unimplemented(e.clone()),
        }
    }
}

impl From<string::FromUtf8Error> for ErrorKind {
    fn from(err: string::FromUtf8Error) -> ErrorKind {
        ErrorKind::Utf8Error(err.utf8_error())
    }
}

impl From<io::Error> for ErrorKind {
    fn from(err: io::Error) -> ErrorKind {
        ErrorKind::Io(err)
    }
}
