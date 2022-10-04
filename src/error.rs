use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "order book error")
        }
    }
}
