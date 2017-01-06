use std::io::{Read,Seek,SeekFrom,Result};

/// Little helper that limits the size of what can be read from a file.
/// Used for reading from pak files in combination with external crates that read until they reach EOF.
pub struct LengthLimitedReader<'a, T:'a+Read+Seek> {
    reader : &'a mut T,
    min_offset: u64,
    max_offset : u64,
}

impl<'a, T:Read+Seek> LengthLimitedReader<'a, T> {
    pub fn new(reader: &mut T, filesize : u64) -> LengthLimitedReader<T> {
        LengthLimitedReader {
            min_offset : reader.seek(SeekFrom::Current(0)).unwrap(),
            max_offset : reader.seek(SeekFrom::Current(0)).unwrap() + filesize,
            reader : reader,
        }
    }
}

impl<'a, T:Read+Seek> Read for LengthLimitedReader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let current_pos = self.reader.seek(SeekFrom::Current(0)).unwrap();
        let pos_after_read = current_pos + buf.len() as u64;
        if pos_after_read > self.max_offset {
            self.reader.read(&mut buf[0 .. (self.max_offset - current_pos) as usize])
        } else {
            self.reader.read(buf)
        }
    }
}

impl<'a, T:Read+Seek> Seek for LengthLimitedReader<'a, T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        // TODO: Check if pos is outside the scope of this reader
        self.reader.seek(pos)
    }
}
