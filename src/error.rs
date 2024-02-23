use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidStartByte(u8),
    InvalidFollowByte(u8),
    InvalidCodepoint(u32),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::InvalidStartByte(byte) => write!(f, "invalid start byte: `{:08b}`", byte),
            Error::InvalidFollowByte(byte) => write!(f, "invalid follow byte: `{:08b}`", byte),
            Error::InvalidCodepoint(cp) => write!(f, "invalid codepoint: `{}`", cp),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
