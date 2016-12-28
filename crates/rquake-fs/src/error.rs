#![warn(missing_docs)]

use std::error;
use std::fmt;
use std::io;

use riff_wave;

/// Read errors when reading a file / resource.
#[derive(Debug)]
pub enum ReadError {
    /// Error from std::io.
    Io(io::Error),
    /// Error from riff_wave
    Wave(riff_wave::ReadError),
    /// Custom error when the read input data is unexpected.
    ParseError,
    /// Custom error when trying to read a file / resource that doesn't exist.
    FileNotFound,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::Io(ref err) => err.fmt(f),
            ReadError::Wave(ref err) => err.fmt(f),
            ReadError::ParseError => write!(f, "Error parsing file"),
            ReadError::FileNotFound => write!(f, "File not found"),
        }
    }
}

impl error::Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::Io(ref err) => err.description(),
            ReadError::Wave(ref err) => err.description(),
            ReadError::ParseError => "parsing error",
            ReadError::FileNotFound => "file not found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ReadError::Io(ref err) => Some(err),
            ReadError::Wave(ref err) => Some(err),
            ReadError::ParseError => None,
            ReadError::FileNotFound => None,
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::Io(err)
    }
}

impl From<riff_wave::ReadError> for ReadError {
    fn from(err: riff_wave::ReadError) -> ReadError {
        ReadError::Wave(err)
    }
}