#![allow(non_upper_case_globals)]
use std::io::{self, Cursor, Read, Seek, Write};

pub mod header;
pub use self::header::*;

pub mod frame;
pub use self::frame::*;

pub mod chunk;
pub use self::chunk::*;

pub mod color;
pub mod helpers;
pub use self::color::*;

/*
https://github.com/aseprite/aseprite/blob/master/docs/ase-file-specs.md

ASE files use Intel (little-endian) byte order.

BYTE (u8): An 8-bit unsigned integer value
WORD (u16): A 16-bit unsigned integer value
SHORT (i16): A 16-bit signed integer value
DWORD (u32): A 32-bit unsigned integer value
LONG (i32): A 32-bit signed integer value
FIXED (f32): A 32-bit fixed point (16.16) value
BYTE[n] ([u8; n]): "n" bytes.
STRING:
    WORD: string length (number of bytes)
    BYTE[length]: characters (in UTF-8) The '\0' character is not included.
PIXEL: One pixel, depending on the image pixel format:
    RGBA: BYTE[4], each pixel have 4 bytes in this order Red, Green, Blue, Alpha.
    Grayscale: BYTE[2], each pixel have 2 bytes in the order Value, Alpha.
    Indexed: BYTE, Each pixel uses 1 byte (the index).
*/

#[derive(Debug, Clone)]
pub struct Aseprite {
    pub header: Header,
    pub frames: Vec<Frame>,
}

impl Aseprite {
    pub fn new(header: Header, frames: Vec<Frame>) -> Self {
        Self { header, frames }
    }

    pub fn from_read<R>(read: &mut R) -> io::Result<Aseprite>
    where
        R: Read + Seek,
    {
        let header = Header::from_read(read)?;
        let mut frames = Vec::with_capacity(header.frames as usize);
        for _ in 0..header.frames {
            frames.push(Frame::from_read(read, &header)?);
        }

        Ok(Self { header, frames })
    }

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        let frames_buf = vec![];
        let mut frames_wtr = Cursor::new(frames_buf);
        for frame in &self.frames {
            frame.write(&mut frames_wtr)?;
        }
        let body_len = frames_wtr.position() as u32;
        self.header.write(wtr, body_len, self.frames.len() as u16)?;
        wtr.write(&frames_wtr.into_inner())?;
        Ok(())
    }
}
