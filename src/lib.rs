#![doc = include_str!("../README.md")] // used for doc tests

pub mod error;
pub use error::Error;
mod buffered_file;
use buffered_file::BufferedFile;

use std::{
    fs::File,
    io::{Bytes, Read},
};

/// The iterator over the chars of a file.
pub struct Fiter<T: Iterator<Item = std::io::Result<u8>>> {
    bytes: T,
}

impl<T: Iterator<Item = std::io::Result<u8>>> Fiter<T> {
    /// Creates a new `Fiter` with the given iterator.
    pub fn new(bytes: T) -> Self {
        Fiter { bytes }
    }
}

impl Fiter<Bytes<File>> {
    /// A helper function to create an unbuffered `Fiter`.
    pub fn new_unbuffered(filename: &str) -> Result<Self, Error> {
        Ok(Fiter::new(File::open(filename)?.bytes()))
    }
}

impl Fiter<BufferedFile<1_000>> {
    /// A helper function to create a buffered `Fiter`.
    pub fn new_buffered(filename: &str) -> Result<Self, Error> {
        Ok(Fiter::new(BufferedFile::new(filename)?))
    }
}

impl<T: Iterator<Item = std::io::Result<u8>>> Iterator for Fiter<T> {
    type Item = Result<char, Error>;

    /// This method implements the conversion of bytes to UTF-8 decoded chars.
    ///
    /// The conversion is implemented as specified in [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629).  
    /// One UTF-8 encoded char can be 1 to 4 bytes long:  
    ///     0xxxxxxx  
    ///     110xxxxx 10xxxxxx  
    ///     1110xxxx 10xxxxxx 10xxxxxx  
    ///     11110xxx 10xxxxxx 10xxxxxx 10xxxxxx  
    /// The first byte indicates how much bytes are following to get a full code point.  
    /// The bits marked as `x` are the data bits, which are used to encode the code point, all other bits are control bits.  
    /// To get a code point as `u32` all control bits are removed and the remaining bits are concatenated together.  
    /// Note that the single byte encoding is equal to ASCII.  
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
            return Some(Err(Error::InvalidStartByte(start_byte)));
        };

        // create code point
        let mut cp = start_byte as u32;
        for _ in 1..units {
            match self.bytes.next()? {
                Err(err) => return Some(Err(err.into())),
                Ok(byte) => {
                    if (byte >> 6) != 0b10 {
                        return Some(Err(Error::InvalidFollowByte(byte)));
                    }
                    cp <<= 6;
                    cp |= (byte & 0b00111111) as u32;
                }
            }
        }

        // convert code point to char
        match char::from_u32(cp).map(Ok) {
            c @ Some(_) => c,
            None => Some(Err(Error::InvalidCodePoint(cp))),
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
        let mut fiter = get_fiter(&['ä', 'Ù', '💚', '😄']);

        assert_eq!(fiter.next().unwrap().unwrap(), 'ä');
        assert_eq!(fiter.next().unwrap().unwrap(), 'Ù');
        assert_eq!(fiter.next().unwrap().unwrap(), '💚');
        assert_eq!(fiter.next().unwrap().unwrap(), '😄');
        assert!(fiter.next().is_none());
    }
}

// run benches with: `cargo test --release -- --nocapture --quiet`
#[cfg(test)]
mod benches {
    use std::{fs::read_to_string, io::BufReader, time::Instant};

    use rstest::rstest;
    use utf8_chars::BufReadCharsExt;

    use super::*;

    #[rstest]
    #[case("benches/large_ascii.txt")]
    #[case("benches/large_utf8.txt")]
    fn large_ascii_std(#[case] file: &str) {
        let now = Instant::now();

        for _ in 0..10 {
            for _ in read_to_string(file).unwrap().chars() {}
        }

        eprintln!("BENCH: std took for `{}`: {:.2?}", file, now.elapsed());
    }

    #[rstest]
    #[case("benches/large_ascii.txt")]
    #[case("benches/large_utf8.txt")]
    fn large_ascii_utf8_chars(#[case] file: &str) {
        let now = Instant::now();

        for _ in 0..10 {
            for c in BufReader::new(File::open(file).unwrap()).chars() {
                c.unwrap();
            }
        }

        eprintln!(
            "BENCH: utf8-chars took for `{}`: {:.2?}",
            file,
            now.elapsed()
        );
    }

    #[rstest]
    #[case("benches/large_ascii.txt")]
    #[case("benches/large_utf8.txt")]
    fn large_ascii_fiter_1k(#[case] file: &str) {
        let now = Instant::now();

        for _ in 0..10 {
            for c in Fiter::new(BufferedFile::<1_000>::new(file).unwrap()) {
                c.unwrap();
            }
        }

        eprintln!("BENCH: fiter_1k took for `{}`: {:.2?}", file, now.elapsed());
    }

    #[rstest]
    #[case("benches/large_ascii.txt")]
    #[case("benches/large_utf8.txt")]
    fn large_ascii_fiter_100k(#[case] file: &str) {
        let now = Instant::now();

        for _ in 0..10 {
            for c in Fiter::new(BufferedFile::<100_000>::new(file).unwrap()) {
                c.unwrap();
            }
        }

        eprintln!(
            "BENCH: fiter_100k took for `{}`: {:.2?}",
            file,
            now.elapsed()
        );
    }
}
