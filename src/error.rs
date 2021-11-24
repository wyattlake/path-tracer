use ocl;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Ocl(ocl::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Io: {:?}\n{}", e.kind(), e),
            Error::Ocl(e) => write!(f, "Ocl:\n{}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ocl::Error> for Error {
    fn from(e: ocl::Error) -> Self {
        Error::Ocl(e)
    }
}
