use std::io::{Cursor, Error as IoError, Read, Seek, SeekFrom};
use std::{io, result};
use std::string::FromUtf8Error;

type Result<T> = result::Result<T, DecodeError>;
pub struct Reader(Cursor<Vec<u8>>);

impl Reader {
    pub fn new(inner: Vec<u8>) -> Self {
        Reader(Cursor::new(inner))
    }

    fn next(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8];
        self.0.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    pub fn next_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0u8;2];
        self.0.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    pub fn next_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0u8;4];
        self.0.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> result::Result<usize, IoError> {
        self.0.read(buf)
    }
}

impl Seek for Reader {
    fn seek(&mut self, pos: SeekFrom) -> result::Result<u64, IoError> {
        self.0.seek(pos)
    }
}

#[derive(Debug)]
pub enum DecodeError {
    Io(IoError),
    Utf8(FromUtf8Error),
}

impl From<IoError> for DecodeError {
    fn from(err: IoError) -> DecodeError {
        DecodeError::Io(err)
    }
}

impl From<FromUtf8Error> for DecodeError {
    fn from(err: FromUtf8Error) -> DecodeError {
        DecodeError::Utf8(err)
    }
}

pub trait NameDecoder {
    fn decode_name(&mut self) -> Result<String>;
    fn decode_compressed_name(&mut self, length: u8) -> Result<String>;
}

impl NameDecoder for Reader {
    fn decode_name(&mut self) -> Result<String> {
        let mut parts: Vec<String> = Vec::new();
        while let Ok(length) = self.next() {
            if length == 0 {
                break;
            }
            if length & 0b1100_0000 != 0 {
                parts.push(self.decode_compressed_name(length)?);
                break;
            } else {
                let mut buffer = vec![0; length as usize];
                self.0.read_exact(&mut buffer)?;
                parts.push(String::from_utf8(buffer)?);
            }
        }
        let name = parts.join(".");
        Ok(name)
    }

    fn decode_compressed_name(&mut self, length: u8) -> Result<String> {
        let pointer = ((length as u16) & 0b0011_1111) << 8 | self.next()? as u16;
        let current_pos = self.0.position();
        self.0.seek(SeekFrom::Start(pointer as u64))?;
        let result = self.decode_name()?;
        self.0.seek(SeekFrom::Start(current_pos))?;
        Ok(result)
    }
}
