use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Error;

pub struct BufferedReader {
    reader: BufReader<File>,
}

impl BufferedReader {
    pub fn new(filename: &str) -> Result<Self, Error> {
        Ok(BufferedReader {
            reader: BufReader::new(File::open(filename)?),
        })
    }
}

impl Iterator for BufferedReader {
    type Item = std::io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer = match self.reader.fill_buf() {
            Ok(b) => b,
            Err(err) => return Some(Err(err)),
        };

        let byte = *buffer.first()?;
        self.reader.consume(1);

        Some(Ok(byte))
    }
}
