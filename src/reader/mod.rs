use std::io::Read;

pub struct EventReader<R: Read> {
    source: R,
}

impl<R: Read> EventReader<R> {
    pub fn new(source: R) -> Self {
        EventReader {
            source: source,
        }
    }
}
