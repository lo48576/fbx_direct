//! Contains implementations of FBX parsers.

use std::io::Read;
use crate::reader::error::{Result, Error, ErrorKind};
use crate::reader::{FbxEvent, ParserConfig};
use crate::common::FbxFormatType;
use self::binary::BinaryParser;
use self::ascii::AsciiParser;

mod macros;
mod ascii;
mod binary;

/// Parser state, with sub parser if necessary.
#[derive(Debug, Clone)]
enum ParserState {
    /// Reading magic binary (i.e. the first line).
    Magic,
    /// Reading binary FBX.
    Binary(BinaryParser),
    /// Reading ASCII FBX.
    Ascii(AsciiParser),
}

/// Common state among all sub parsers.
#[derive(Debug, Clone)]
pub(crate) struct CommonState {
    /// Position of last successfully read byte.
    pos: u64,
    final_result: Option<Result<FbxEvent>>,
}

/// A simple wrapper around magic, binary and ascii FBX parser.
pub struct Parser {
    config: ParserConfig,
    common: CommonState,
    state: ParserState,
}

impl Parser {
    /// Constructs a parser.
    pub fn new(config: ParserConfig) -> Self {
        Parser {
            config: config,
            common: CommonState {
                pos: 0,
                final_result: None,
            },
            state: ParserState::Magic,
        }
    }

    /// Get next `FbxEvent`.
    pub fn next<R: Read>(&mut self, reader: &mut R) -> Result<FbxEvent> {
        // If parsing has been finished, return the last result.
        if let Some(ref result) = self.common.final_result {
            return result.clone();
        }
        let result;
        loop {
            // Parsing is not finished, call sub parser.
            let r = match self.state {
                ParserState::Magic => self.magic_next(reader),
                ParserState::Binary(ref mut parser) => parser.next(reader, &mut self.common),
                ParserState::Ascii(ref mut parser) => parser.next(reader, &mut self.common),
            };
            // Break only when `ignore_comments` option is disabled or got non-comment event.
            if self.config.ignore_comments {
                match r {
                    Ok(FbxEvent::Comment(_)) => {},
                    r => {
                        result = r;
                        break;
                    }
                }
            } else {
                result = r;
                break;
            }
        }
        // If parsing is finished, set `final_result`.
        match result {
            Ok(FbxEvent::EndFbx) | Err(_) => {
                self.common.final_result = Some(result.clone());
            },
            _ => {}
        }
        result
    }

    /// Read magic binary and update parser state if success.
    fn magic_next<R: Read>(&mut self, reader: &mut R) -> Result<FbxEvent> {
        // 20 is the length of `b"Kaydara FBX Binary  "`.
        let mut first_line_bytes = Vec::with_capacity(20);
        // First, read the first line.
        // Read the first line manually.
        let magic_end_byte;
        loop {
            let c = try_read_le_u8!(self.common.pos, reader);
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
                // "unknown but all observed files show these bytes",
                // see https://code.blender.org/2013/08/fbx-binary-file-format-specification/ .
                {
                    let bytes = try_read_exact!(self.common.pos, reader, 2);
                    if bytes != vec![0x1A, 0x00] {
                        warn!("expected [0x1A, 0x00] right after magic, but got {:?}", bytes);
                    }
                }
                // Read FBX version.
                let version = try_read_le_u32!(self.common.pos, reader);
                debug!("magic binary read, Binary FBX (version={})", version);
                self.state = ParserState::Binary(BinaryParser::new(version));
                Ok(FbxEvent::StartFbx(FbxFormatType::Binary(version)))
            } else {
                Err(Error::new(self.common.pos, ErrorKind::InvalidMagic))
            }
        } else {
            assert_eq!(magic_end_byte, ('\n' as u8));
            // Maybe ASCII FBX
            let mut buffer;
            if first_line_bytes[0] != (';' as u8) {
                // The line is not comment, so the parser should remember it to use next time.
                buffer = try_with_pos!(self.common.pos, String::from_utf8(first_line_bytes));
                buffer.push('\n');
            } else {
                buffer = String::new();
            }
            self.state = ParserState::Ascii(AsciiParser::new(buffer));
            Ok(FbxEvent::StartFbx(FbxFormatType::Ascii))
        }
    }
}
