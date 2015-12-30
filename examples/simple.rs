extern crate fbx_direct;
extern crate env_logger;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use fbx_direct::reader;
use fbx_direct::reader::EventReader;
use fbx_direct::writer::EventWriter;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn main() {
    env_logger::init().unwrap();

    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            use std::io::Write;
            writeln!(&mut std::io::stderr(), "Usage: cargo run --example=simple <FBX_filename>").unwrap();
            std::process::exit(1);
        },
    };

    let file = BufReader::new(File::open(filename.clone()).unwrap());
    let file_out = BufWriter::new(File::create(filename + ".exported.fbx").unwrap());

    let parser = EventReader::new(file);
    let mut emitter = EventWriter::new(file_out);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(ref e@reader::FbxEvent::StartNode { .. }) => {
                println!("{}{:?}", indent(depth), e);
                depth += 1;
            },
            Ok(ref e@reader::FbxEvent::EndNode) => {
                depth -= 1;
                println!("{}{:?}", indent(depth), e);
            },
            Ok(ref e) => {
                println!("{}{:?}", indent(depth), e);
            },
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            },
        }
        if let Ok(ref e) = e {
            emitter.write(e.as_writer_event()).unwrap();
        }
    }
}
