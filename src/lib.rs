//! This crate currently provides pull parser for Binary FBX.
//!
//! FBX data consists of generic node and node properties, and it requires interpretation to use as
//! 3D contents.
//! It is similar to relation of XML and COLLADA. COLLADA is represented using XML, but XML DOM is
//! difficult to use directly as COLLADA data.
//! Compare FBX to COLLADA, this crate is XML reader/writer, not COLLADA importer/exporter.

extern crate base64;
extern crate byteorder;
#[macro_use]
extern crate log;

pub use crate::reader::EventReader;
pub use crate::writer::EventWriter;

pub mod common;
pub mod reader;
pub mod writer;
