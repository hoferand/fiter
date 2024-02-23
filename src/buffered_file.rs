use std::{fs::File, io::Read};

use crate::Error;

pub struct BufferedFile<const B: usize> {
    file: File,
    buf: [u8; B],
    pt: usize,
    max: usize,
}

impl<const B: usize> BufferedFile<B> {
    pub fn new(filename: &str) -> Result<Self, Error> {
        Ok(BufferedFile {
            file: File::open(filename)?,
            buf: [0; B],
            pt: 0,
            max: 0,
        })
    }

    fn load(&mut self) -> std::io::Result<()> {
        self.max = self.file.read(&mut self.buf)?;
        self.pt = 0;
        Ok(())
    }
}

impl<const B: usize> Iterator for BufferedFile<B> {
    type Item = std::io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pt >= self.max {
            match self.load() {
                Err(err) => return Some(Err(err)),
                _ => (),
            }
        }

        if self.pt < self.max {
            let byte = self.buf[self.pt];
            self.pt += 1;
            Some(Ok(byte))
        } else {
            None
        }
    }
}
