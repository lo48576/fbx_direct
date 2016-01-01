extern crate fbx_direct;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::io::BufReader;

use fbx_direct::reader;
use fbx_direct::reader::EventReader;
use fbx_direct::writer::EventWriter;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn main() {
    use std::io::Write;

    env_logger::init().unwrap();

    let filename = "assets/blender_2_72b_default.fbx";

    let mut exported1 = std::io::Cursor::new(Vec::<u8>::new());
    {
        let file = BufReader::new(File::open(filename).unwrap());
        let parser = EventReader::new(file);
        let mut emitter = EventWriter::new(exported1.by_ref());
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(ref e@reader::FbxEvent::StartNode { .. }) => {
                    debug!("{}{:?}", indent(depth), e);
                    depth += 1;
                },
                Ok(ref e@reader::FbxEvent::EndNode) => {
                    depth -= 1;
                    debug!("{}{:?}", indent(depth), e);
                },
                Ok(ref e) => {
                    debug!("{}{:?}", indent(depth), e);
                },
                Err(e) => {
                    debug!("Error: {:?}", e);
                    break;
                },
            }
            if let Ok(ref e) = e {
                emitter.write(e.as_writer_event()).unwrap();
            }
        }
    }
    {
        use std::io::{Seek,SeekFrom};
        exported1.seek(SeekFrom::Start(0)).unwrap();
    }

    let mut exported2 = std::io::Cursor::new(Vec::<u8>::new());
    {
        let parser = EventReader::new(exported1.by_ref());
        let mut emitter = EventWriter::new(exported2.by_ref());
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(ref e@reader::FbxEvent::StartNode { .. }) => {
                    debug!("{}{:?}", indent(depth), e);
                    depth += 1;
                },
                Ok(ref e@reader::FbxEvent::EndNode) => {
                    depth -= 1;
                    debug!("{}{:?}", indent(depth), e);
                },
                Ok(ref e) => {
                    debug!("{}{:?}", indent(depth), e);
                },
                Err(e) => {
                    debug!("Error: {:?}", e);
                    break;
                },
            }
            if let Ok(ref e) = e {
                emitter.write(e.as_writer_event()).unwrap();
            }
        }
    }

    assert_eq!(exported1.get_ref(), exported2.get_ref());
}
