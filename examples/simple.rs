extern crate fbx_direct;

use std::fs::File;
use std::io::BufReader;

use fbx_direct::reader::{EventReader};

fn main() {
    let file = BufReader::new(File::open("sample.fbx").unwrap());

    let _parser = EventReader::new(file);
}
