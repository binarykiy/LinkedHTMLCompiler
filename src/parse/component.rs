use std::fmt::{Display, Formatter};
use std::{fmt, mem};
use crate::parse::doc::Doc;
use crate::parse::tag::{BinaryTag, Tag};

#[derive(Debug)]
pub enum BinaryComponent {
    Text(Vec<u8>),
    Comment(Vec<u8>),
    CustomComment(Vec<u8>),
    Tag(BinaryTag),
    CustomTag(BinaryTag, Vec<u8>),
    DocType(Vec<u8>),
    Pointer(Doc),
    Null,
}

#[derive(Debug)]
pub enum Component {
    Text(String),
    Comment(String),
    CustomComment(String),
    Tag(Tag),
    CustomTag(Tag, String),
    DocType(String),
    Pointer(Doc),
    Null,
}

impl Component {
    pub fn swap_null(&mut self) -> Self {
        let mut dest = Self::Null;
        mem::swap(self, &mut dest);
        dest
    }
}

impl Display for Component {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(v) => {
                write!(fmt, "{}", v)
            }
            Self::Comment(v) => {
                write!(fmt, "<!--{}-->", v)
            }
            Self::CustomComment(v) => {
                write!(fmt, "<!--?{}-->", v)
            }
            Self::Tag(v) => {
                write!(fmt, "<{}>", v)
            }
            Self::CustomTag(_, v) => {
                write!(fmt, "<!--?{}-->", v)
            }
            Self::DocType(v) => {
                write!(fmt, "<!{}>", v)
            }
            Self::Pointer(v) => {
                write!(fmt, "{}", v)
            }
            Self::Null => {
                // null cannot format
                Err(fmt::Error)
            }
        }
    }
}
