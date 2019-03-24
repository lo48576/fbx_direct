use env_logger;

use std::fs::File;
use std::io::BufReader;

use fbx_direct::reader::{EventReader, FbxEvent};

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

    let file = BufReader::new(File::open(filename.clone()).unwrap());

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(ref e @ FbxEvent::StartNode { .. }) => {
                println!("{}{:?}", indent(depth), e);
                depth += 1;
            }
            Ok(ref e @ FbxEvent::EndNode) => {
                depth -= 1;
                println!("{}{:?}", indent(depth), e);
            }
            Ok(ref e) => {
                println!("{}{:?}", indent(depth), e);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}
