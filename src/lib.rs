#[macro_use]
extern crate log;

pub mod error;
pub mod reader;

pub type Result<T> = std::result::Result<T, error::Error>;

#[test]
fn it_works() {
}
