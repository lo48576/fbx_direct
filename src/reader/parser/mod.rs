extern crate byteorder;

use std::io::Read;
use error::{Result, Error, ErrorKind};
use reader::{FbxEvent, FbxFormatType};

mod macros;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParserMode {
    /// Reading magic binary (i.e. the first line)
    Magic,
    /// Reading binary FBX
    Binary,
    /// Reading ASCII FBX
    Ascii,
}

pub struct Parser {
    pos: u64,
    mode: ParserMode,
    ascii_buffer: String,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            pos: 0,
            mode: ParserMode::Magic,
            ascii_buffer: String::new(),
        }
    }

    pub fn next<R: Read>(&mut self, reader: &mut R) -> Result<FbxEvent> {
        match self.mode {
            ParserMode::Magic => self.magic_next(reader),
            ParserMode::Binary => self.binary_next(reader),
            ParserMode::Ascii => self.ascii_next(reader),
        }
    }

    pub fn magic_next<R: Read>(&mut self, reader: &mut R) -> Result<FbxEvent> {
        // 20 is the length of `b"Kaydara FBX Binary  "`.
        let mut first_line_bytes = Vec::with_capacity(20);
        // First, read the first line.
        // Read the first line manually.
        let magic_end_byte;
        loop {
            let c = try_read_le_u8!(self.pos, reader);
            if (c == 0) || (c == ('\n' as u8)) {
                magic_end_byte = c;
                break;
            }
            first_line_bytes.push(c);
        }
        // In Binary FBX, magic binary is `"Kaydara FBX Binary  "`,
        // and in ASCII FBX, there is no magic and it should be treated as normal line.
        if magic_end_byte == 0 {
            // Binary FBX?
            if first_line_bytes == b"Kaydara FBX Binary  " {
                // Binary FBX!
                self.mode = ParserMode::Binary;
                // "unknown but all observed files show these bytes",
                // see https://code.blender.org/2013/08/fbx-binary-file-format-specification/ .
                {
                    let bytes = try_read_exact!(self.pos, reader, 2);
                    if bytes != vec![0x1A, 0x00] {
                        warn!("expected [0x1A, 0x00] right after magic, but got {:?}", bytes);
                    }
                }
                // Read FBX version.
                let version = try_read_le_u32!(self.pos, reader);
                debug!("magic binary read, Binary FBX (version={})", version);
                Ok(FbxEvent::StartFbx(FbxFormatType::Binary(version)))
            } else {
                Err(Error::new(self.pos, ErrorKind::InvalidMagic))
            }
        } else {
            assert_eq!(magic_end_byte, ('\n' as u8));
            // Maybe ASCII FBX
            self.mode = ParserMode::Ascii;
            if first_line_bytes[0] != (';' as u8) {
                // The line is not comment, so the parser should remember it to use next time.
                self.ascii_buffer = try_with_pos!(self.pos, String::from_utf8(first_line_bytes));
                self.ascii_buffer.push('\n');
            }
            Ok(FbxEvent::StartFbx(FbxFormatType::Text))
        }
    }

    pub fn binary_next<R: Read>(&mut self, _reader: &mut R) -> Result<FbxEvent> {
        Err(Error::new(self.pos, ErrorKind::Unimplemented("Parser for Binary FBX format is not implemented yet".to_string())))
    }

    pub fn ascii_next<R: Read>(&mut self, _reader: &mut R) -> Result<FbxEvent> {
        Err(Error::new(self.pos, ErrorKind::Unimplemented("Parser for ASCII FBX format is not implemented yet".to_string())))
    }
}
