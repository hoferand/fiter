pub mod error;
pub use error::Error;
mod buffered_file;
use buffered_file::BufferedFile;
mod buffered_reader;
use buffered_reader::BufferedReader;

use std::{
    fs::File,
    io::{Bytes, Read},
};

pub struct Fiter<T: Iterator<Item = std::io::Result<u8>>> {
    bytes: T,
}

impl<T: Iterator<Item = std::io::Result<u8>>> Fiter<T> {
    pub fn new(bytes: T) -> Self {
        Fiter { bytes }
    }
}

impl Fiter<Bytes<File>> {
    pub fn new_no_buf(filename: &str) -> Result<Self, Error> {
        Ok(Fiter::new(File::open(filename)?.bytes()))
    }
}

impl Fiter<BufferedFile<1000>> {
    pub fn new_buf_file(filename: &str) -> Result<Self, Error> {
        Ok(Fiter::new(BufferedFile::new(filename)?))
    }
}

impl Fiter<BufferedReader> {
    pub fn new_buf_reader(filename: &str) -> Result<Self, Error> {
        Ok(Fiter::new(BufferedReader::new(filename)?))
    }
}

impl<T: Iterator<Item = std::io::Result<u8>>> Iterator for Fiter<T> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // get start byte
        let mut start_byte = match self.bytes.next()? {
            Ok(byte) => byte,
            Err(err) => return Some(Err(err.into())),
        };
        let units = if start_byte >> 7 == 0 {
            1
        } else if start_byte >> 5 == 0b110 {
            start_byte &= 0b00011111;
            2
        } else if start_byte >> 4 == 0b1110 {
            start_byte &= 0b00001111;
            3
        } else if start_byte >> 3 == 0b11110 {
            start_byte &= 0b00000111;
            4
        } else {
            return Some(Err(Error::InvalidByte(start_byte)));
        };

        // create codepoint
        let mut cp = start_byte as u32;
        for _ in 0..(units - 1) {
            match self.bytes.next()? {
                Err(err) => return Some(Err(err.into())),
                Ok(byte) => {
                    if (byte >> 6) != 0b10 {
                        return Some(Err(Error::InvalidByte(byte)));
                    }
                    cp <<= 6;
                    cp |= (byte & 0b00111111) as u32;
                }
            }
        }

        // convert codepoint to char
        match char::from_u32(cp).map(|c| Ok(c)) {
            c @ Some(_) => c,
            None => Some(Err(Error::InvalidUnicode(cp))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticBytes {
        bytes: Vec<u8>,
        pt: usize,
    }

    impl Iterator for StaticBytes {
        type Item = std::io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            let val = *self.bytes.get(self.pt)?;
            self.pt += 1;

            Some(Ok(val))
        }
    }

    fn get_fiter(chars: &[char]) -> Fiter<StaticBytes> {
        let mut bytes = Vec::new();

        for ch in chars {
            let mut buf = vec![0; ch.len_utf8()];
            ch.encode_utf8(&mut buf);
            bytes.push(buf);
        }

        Fiter::new(StaticBytes {
            bytes: bytes.iter().flatten().copied().collect(),
            pt: 0,
        })
    }

    #[test]
    fn single_byte() {
        let mut fiter = get_fiter(&['a', 'A', '0', '\0']);

        assert_eq!(fiter.next().unwrap().unwrap(), 'a');
        assert_eq!(fiter.next().unwrap().unwrap(), 'A');
        assert_eq!(fiter.next().unwrap().unwrap(), '0');
        assert_eq!(fiter.next().unwrap().unwrap(), '\0');
        assert!(fiter.next().is_none());
    }

    #[test]
    fn multi_byte() {
        let mut fiter = get_fiter(&['ä', 'Ü', '💚', '😄']);

        assert_eq!(fiter.next().unwrap().unwrap(), 'ä');
        assert_eq!(fiter.next().unwrap().unwrap(), 'Ü');
        assert_eq!(fiter.next().unwrap().unwrap(), '💚');
        assert_eq!(fiter.next().unwrap().unwrap(), '😄');
        assert!(fiter.next().is_none());
    }
}
