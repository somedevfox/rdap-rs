use std::io;

#[derive(Debug)]
pub enum Error {
    Request(ureq::Error),
    Io(io::Error),
    
    NotDomain
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Self::Request(value)
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;