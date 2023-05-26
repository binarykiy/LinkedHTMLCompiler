use std::fmt::{Display, Formatter};
use std::{fmt, mem};
use crate::parse::doc::Doc;
use crate::parse::tag::Tag;

#[derive(Debug)]
pub enum Token {
    Text(String),
    Comment(String),
    Tag(Tag),
    CustomTag(Tag),
    DocType(String),
    Pointer(Doc),
    Null,
}

impl Token {
    pub fn swap_null(&mut self) -> Self {
        let mut dest = Self::Null;
        mem::swap(self, &mut dest);
        dest
    }
}

impl Display for Token {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(v) => {
                write!(fmt, "{}", v)
            }
            Self::Comment(v) => {
                write!(fmt, "<!--{}-->", v)
            }
            Self::Tag(v) => {
                write!(fmt, "<{}>", v)
            }
            Self::CustomTag(v) => {
                write!(fmt, "<!--?{}-->", v)
            }
            Self::DocType(v) => {
                write!(fmt, "<!{}>", v)
            }
            Self::Pointer(v) => {
                for i in 0..v.len() {
                    write!(fmt, "{}", v[i])?;
                }
                Ok(())
            }
            Self::Null => {
                // null cannot format
                Err(fmt::Error)
            }
        }
    }
}