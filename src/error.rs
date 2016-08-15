use std::error;
use std::fmt;

/// Error type for dealing with problems
/// with the encoder.
#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    OutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OutOfBounds => write!(f, "Tetromino out of bounds"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OutOfBounds => "tetromino out of bounds",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::OutOfBounds => None,
        }
    }
}
