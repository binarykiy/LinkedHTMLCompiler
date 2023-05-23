mod include;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
pub use include::run as include;

#[derive(Debug)]
pub enum SyntaxError {
    NoTagBegin(String),
    NoTagEnd(String),
    IsNotClosed(String),
}

impl Display for SyntaxError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTagBegin(s) => {
                write!(fmt, "Expected {}, not found", s)
            }
            Self::NoTagEnd(s) => {
                write!(fmt, "Expected {}, not found", s)
            }
            Self::IsNotClosed(s) => {
                write!(fmt, "Expected > for {}, not found", s)
            }
        }
    }
}

impl Error for SyntaxError {
}

pub fn extract<'a, 'b>(source: &'a str, tag: &'b str) -> Result<&'a str, SyntaxError> {
    let opened_begin = format!("<{}", tag);
    if let Some((_, from_prefix)) = source.split_once(&opened_begin) {
        if let Some((_, after_prefix)) = from_prefix.split_once('>') {
            let end = format!("</{}>", tag);
            if let Some((res, _)) = after_prefix.rsplit_once(&end) {
                Ok(res)
            } else {
                Err(SyntaxError::NoTagEnd(end))
            }
        } else {
            Err(SyntaxError::IsNotClosed(opened_begin))
        }
    } else {
        Err(SyntaxError::NoTagBegin(opened_begin))
    }
}
