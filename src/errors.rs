use std::error;
use std::fmt;

pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self{message}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{ message: {} }}", self.message)
    }
}

// Implement the Error trait
impl error::Error for Error {}

// Implement the From trait to convert a standard library Error to custom Error
impl From<Box<dyn error::Error>> for Error {
    fn from(err: Box<dyn error::Error>) -> Self {
        Error {
            message: format!("{}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            message: format!("{}", err),
        }
    }
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Error {
            message: format!("Git error: {}", error),
        }
    }
}
