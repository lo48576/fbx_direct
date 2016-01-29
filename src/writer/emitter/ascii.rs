//! Contains implementation of ASCII FBX emitter.
extern crate rustc_serialize;

use std::io::Write;
use writer::error::{Result, Error};
use common::Property;

fn indent<W: Write>(sink: &mut W, depth: usize) -> Result<()> {
    for _ in 0..depth {
        try!(sink.write(b"\t"));
    }
    Ok(())
}

fn print_property<W: Write>(sink: &mut W, property: &Property, prop_depth: usize) -> Result<()> {
    assert!(prop_depth > 0);

    // TODO: I've never seen vector of booleans (in binary or ascii FBX)... How should it be?
    // TODO: How will it be when other properties follows a property of array value?
    // TODO: Implement folding of large array.
    match *property {
        Property::Bool(false) => {
            try!(sink.write(b"T"));
        },
        Property::Bool(true) => {
            try!(sink.write(b"Y"));
        },
        Property::I16(v) => {
            try!(sink.write_fmt(format_args!("{}", v)));
        },
        Property::I32(v) => {
            try!(sink.write_fmt(format_args!("{}", v)));
        },
        Property::I64(v) => {
            try!(sink.write_fmt(format_args!("{}", v)));
        },
        Property::F32(v) => {
            // NOTE: Is outputted data accurate enough?
            try!(sink.write_fmt(format_args!("{}", v)));
        },
        Property::F64(v) => {
            // NOTE: Is outputted data accurate enough?
            try!(sink.write_fmt(format_args!("{}", v)));
        },
        Property::VecBool(vec) => {
            warn!("ASCII representation of vector of boolean values may be wrong.");
            try!(sink.write_fmt(format_args!("*{} {{\n", vec.len())));
            try!(indent(sink, prop_depth));
            try!(sink.write(b"a: "));
            let mut iter = vec.iter();
            if let Some(&v) = iter.next() {
                try!(sink.write(if v { b"Y" } else { b"T" }));
            }
            for &v in iter {
                try!(sink.write(if v { b",Y" } else { b",T" }));
            }
            try!(sink.write(b"\n"));
            try!(indent(sink, prop_depth-1));
            try!(sink.write(b"}"));
        },
        Property::VecI32(vec) => {
            try!(sink.write_fmt(format_args!("*{} {{\n", vec.len())));
            try!(indent(sink, prop_depth));
            try!(sink.write(b"a: "));
            let mut iter = vec.iter();
            if let Some(&v) = iter.next() {
                try!(sink.write_fmt(format_args!("{}", v)));
            }
            for &v in iter {
                try!(sink.write_fmt(format_args!(",{}", v)));
            }
            try!(sink.write(b"\n"));
            try!(indent(sink, prop_depth-1));
            try!(sink.write(b"}"));
        },
        Property::VecI64(vec) => {
            try!(sink.write_fmt(format_args!("*{} {{\n", vec.len())));
            try!(indent(sink, prop_depth));
            try!(sink.write(b"a: "));
            let mut iter = vec.iter();
            if let Some(&v) = iter.next() {
                try!(sink.write_fmt(format_args!("{}", v)));
            }
            for &v in iter {
                try!(sink.write_fmt(format_args!(",{}", v)));
            }
            try!(sink.write(b"\n"));
            try!(indent(sink, prop_depth-1));
            try!(sink.write(b"}"));
        },
        Property::VecF32(vec) => {
            try!(sink.write_fmt(format_args!("*{} {{\n", vec.len())));
            try!(indent(sink, prop_depth));
            try!(sink.write(b"a: "));
            let mut iter = vec.iter();
            if let Some(&v) = iter.next() {
                try!(sink.write_fmt(format_args!("{}", v)));
            }
            for &v in iter {
                try!(sink.write_fmt(format_args!(",{}", v)));
            }
            try!(sink.write(b"\n"));
            try!(indent(sink, prop_depth-1));
            try!(sink.write(b"}"));
        },
        Property::VecF64(vec) => {
            try!(sink.write_fmt(format_args!("*{} {{\n", vec.len())));
            try!(indent(sink, prop_depth));
            try!(sink.write(b"a: "));
            let mut iter = vec.iter();
            if let Some(&v) = iter.next() {
                try!(sink.write_fmt(format_args!("{}", v)));
            }
            for &v in iter {
                try!(sink.write_fmt(format_args!(",{}", v)));
            }
            try!(sink.write(b"\n"));
            try!(indent(sink, prop_depth-1));
            try!(sink.write(b"}"));
        },
        Property::String(v) => {
            try!(sink.write(b"\""));
            for c in v.chars() {
                match c {
                    '"' => {
                        try!(sink.write(b"&quot;"));
                    },
                    '\n' => {
                        try!(sink.write(b"&lf;"));
                    },
                    '\r' => {
                        try!(sink.write(b"&cr;"));
                    },
                    _ => {
                        try!(sink.write_fmt(format_args!("{}", c)));
                    }
                }
            }
            try!(sink.write(b"\""));
        },
        Property::Binary(v) => {
            // TODO: Implement folding of long line.
            use self::rustc_serialize::base64::{ToBase64, STANDARD};
            // base64 conversion.
            try!(sink.write_fmt(format_args!("\"{}\"", v.to_base64(STANDARD))));
        },
    }
    Ok(())
}

/// A writer for ASCII FBX.
#[derive(Debug, Clone)]
pub struct AsciiEmitter {
    prop_child_existence: Vec<(bool, bool)>,
}

impl AsciiEmitter {
    /// Constructs ASCII FBX writer.
    pub fn new() -> Self {
        AsciiEmitter {
            prop_child_existence: vec![],
        }
    }

    pub fn emit_start_fbx<W: Write>(&mut self, sink: &mut W, ver: u32) -> Result<()> {
        if (ver < 7000) || (ver >= 8000) {
            error!("Unsupported version: {}", ver);
            return Err(Error::UnsupportedFbxVersion(ver));
        }
        {
            let (major, minor) = (ver / 1000, ver % 1000);
            let (minor, revision) = (minor / 100, minor % 100);
            // Write magic for ASCII FBX.
            try!(sink.write_fmt(format_args!("; FBX {}.{}.{} project file\n", major, minor, revision)));
        }

        Ok(())
    }

    pub fn emit_end_fbx<W: Write>(&mut self, _sink: &mut W) -> Result<()> {
        Ok(())
    }

    pub fn emit_start_node<W: Write>(&mut self, sink: &mut W, name: &str, properties: &[Property]) -> Result<()> {
        if let Some((prop_exist, child_exist)) = self.prop_child_existence.pop() {
            // Print brace for *parent node*, if the current node is the first child.
            // (i.e. `child_exist` of parent is `false`.)
            if !child_exist {
                try!(sink.write(b" {\n"));
            }
            self.prop_child_existence.push((prop_exist, true));
        }
        try!(indent(sink, self.prop_child_existence.len()));
        self.prop_child_existence.push((!properties.is_empty(), false));
        try!(sink.write_fmt(format_args!("{}: ", name)));

        let prop_depth = self.prop_child_existence.len();
        let mut prop_iter = properties.iter();
        if let Some(prop) = prop_iter.next() {
            //try!(sink.write_fmt(format_args!("{:?}", prop)));
            try!(print_property(sink, prop, prop_depth));
        }
        for prop in prop_iter {
            //try!(sink.write_fmt(format_args!(", {:?}", prop)));
            try!(sink.write(b", "));
            try!(print_property(sink, prop, prop_depth));
        }

        Ok(())
    }

    pub fn emit_end_node<W: Write>(&mut self, sink: &mut W) -> Result<()> {
        let (prop_exist, child_exist) = self.prop_child_existence.pop().unwrap();
        if !prop_exist || child_exist {
            if !prop_exist && !child_exist {
                try!(sink.write(b" {\n"));
            }
            try!(indent(sink, self.prop_child_existence.len()));
            try!(sink.write(b"}\n"));
        } else {
            try!(sink.write(b"\n"));
        }

        Ok(())
    }

    pub fn emit_comment<W: Write>(&mut self, sink: &mut W, comment: &str) -> Result<()> {
        for line in comment.lines() {
            try!(indent(sink, self.prop_child_existence.len()));
            try!(sink.write(line.as_bytes()));
            try!(sink.write(b"\n"));
        }

        Ok(())
    }
}
