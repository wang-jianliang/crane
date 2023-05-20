use std::fmt;

pub struct Error {
    pub message: String,
}

// 根据错误码显示不同的错误信息
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
