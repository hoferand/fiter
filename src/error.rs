use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidByte(u8),
    InvalidUnicode(u32),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::InvalidByte(byte) => write!(f, "{:08b}", byte),
            Error::InvalidUnicode(unicode) => write!(f, "{}", unicode),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
