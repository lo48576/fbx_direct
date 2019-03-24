use fbx_direct;
#[macro_use]
extern crate log;
use env_logger;

use std::fs::File;
use std::io::BufReader;

use fbx_direct::reader::EventReader;
use fbx_direct::reader::FbxEvent as ReaderEvent;
use fbx_direct::writer::EmitterConfig;
use fbx_direct::writer::FbxEvent as WriterEvent;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn main() {
    env_logger::init();

    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            use std::io::Write;
            writeln!(
                &mut std::io::stderr(),
                "Usage: cargo run --example=simple <FBX_filename>"
            )
            .unwrap();
            std::process::exit(1);
        }
    };
    let new_filename = filename.clone() + ".ascii.fbx";

    let file = BufReader::new(File::open(filename.clone()).unwrap());
    let parser = EventReader::new(file);
    let mut emitter = EmitterConfig::new()
        .fbx_version(Some(7500))
        .create_writer(File::create(new_filename.clone()).unwrap());
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(ref e @ ReaderEvent::StartNode { .. }) => {
                debug!("{}{:?}", indent(depth), e);
                depth += 1;
            }
            Ok(ref e @ ReaderEvent::EndNode) => {
                depth -= 1;
                debug!("{}{:?}", indent(depth), e);
            }
            Ok(ref e) => {
                debug!("{}{:?}", indent(depth), e);
            }
            Err(e) => {
                debug!("Error: {:?}", e);
                break;
            }
        }
        match e {
            Ok(ReaderEvent::StartFbx(_)) => {
                emitter
                    .write(WriterEvent::StartFbx(
                        fbx_direct::common::FbxFormatType::Ascii,
                    ))
                    .unwrap();
            }
            Ok(ref e) => {
                emitter.write(e.as_writer_event()).unwrap();
            }
            _ => unreachable!(),
        }
    }

    println!("written to {}", new_filename);
}
