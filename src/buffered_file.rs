use std::{fs::File, io::Read};

use crate::Error;

/// A buffered iterator over the bytes of a file.
pub struct BufferedFile<const B: usize> {
    file: File,
    buf: [u8; B],
    pt: usize,
    max: usize,
}

impl<const B: usize> BufferedFile<B> {
    /// Opens the given filename and creates a new `BufferedFile` from it.
    pub fn new(filename: &str) -> Result<Self, Error> {
        Ok(BufferedFile {
            file: File::open(filename)?,
            buf: [0; B],
            pt: 0,
            max: 0,
        })
    }

    /// Refills the buffer.
    ///
    /// This method should only be called if the buffer is empty,
    /// because the whole buffer gets overridden.
    fn load(&mut self) -> std::io::Result<()> {
        self.max = self.file.read(&mut self.buf)?;
        self.pt = 0;
        Ok(())
    }
}

impl<const B: usize> Iterator for BufferedFile<B> {
    type Item = std::io::Result<u8>;

    /// Returns the next byte of this file.
    fn next(&mut self) -> Option<Self::Item> {
        if self.pt >= self.max {
            // refill buffer
            if let Err(err) = self.load() {
                return Some(Err(err));
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
