use std::fmt::Display;

/// The error thrown by the iterator.
#[derive(Debug)]
pub enum Error {
    /// Is thrown if an io error occurs during reading the file.
    Io(std::io::Error),
    /// Is thrown if a start byte is expected,
    /// but the byte has not an appropriate bit sequence.
    /// `offset` is the number of bytes before the faulty byte.
    InvalidStartByte { offset: u64, byte: u8 },
    /// Is thrown if a following byte do not start with `0b10`.
    /// `offset` is the number of bytes before the faulty byte.
    InvalidFollowByte { offset: u64, byte: u8 },
    /// Is thrown if a decoded byte sequence results in an invalid code point.
    /// `offset` is the number of bytes before the starting byte of the faulty code point.
    InvalidCodePoint { offset: u64, cp: u32 },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "io error `{}`", err),
            Error::InvalidStartByte { offset, byte } => {
                write!(f, "invalid start byte `{:08b}` at `{}`", byte, offset)
            }
            Error::InvalidFollowByte { offset, byte } => {
                write!(f, "invalid follow byte `{:08b}` at `{}`", byte, offset)
            }
            Error::InvalidCodePoint { offset, cp } => {
                write!(f, "invalid code point `{}` at `{}`", cp, offset)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
