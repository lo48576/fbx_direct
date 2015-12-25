extern crate fbx_direct;

use std::fs::File;
use std::io::BufReader;

use fbx_direct::reader::{EventReader, FbxEvent};

fn main() {
    let file = BufReader::new(File::open("sample.fbx").unwrap());

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            },
            _ => {
                println!("{:?}", e);
            }
        }
    }
}
