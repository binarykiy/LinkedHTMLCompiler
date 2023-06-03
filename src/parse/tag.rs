use std::fmt::{Display, Formatter};
use std::{fmt, mem, str};
use crate::source::SourceManager;
use crate::util;
use crate::util::VecDict;

#[derive(Debug)]
pub struct BinaryTag {
    tag: Vec<u8>,
    attributes: VecDict<Vec<u8>, Vec<u8>>,
}

impl BinaryTag {
    pub fn new(source: &mut SourceManager) -> Option<Self> {
        // todo
        None
    }
    pub fn new_custom(source: SourceManager) -> Option<Self> {
        // todo
        None
    }
}

#[derive(Debug)]
pub struct Tag {
    tag: String,
    attributes: VecDict<String, String>,
}

impl Tag {
    pub fn new(str_inside: &str) -> Option<Self> {
        let (tag, raw_attr) = str_inside.split_once(' ')
            .unwrap_or((str_inside, ""));
        let mut res = Self {
            tag: String::from(tag),
            attributes: VecDict::new(),
        };
        let attributes = raw_attr.as_bytes();
        let len = attributes.len();
        let mut idx = util::first_not_of(attributes, b' ', 0);
        while idx < len {
            idx = res.next_attribute(attributes, idx)?;
        }
        Some(res)
    }
    pub fn new_once(str_all: &mut &str) -> Option<Self> {
        if !str_all.starts_with("<") {
            return None
        }
        (_, *str_all) = str_all.split_once("<").unwrap();
        let (tag, raw_attr) = str_all.split_once('>')?;
        if !tag.contains(' ') {
            *str_all = raw_attr;
            return Self::new(tag)
        }
        let (tag, raw_attr) = str_all.split_once(' ').unwrap();
        let mut res = Self {
            tag: String::from(tag),
            attributes: VecDict::new(),
        };
        let attributes = raw_attr.as_bytes();
        let len = attributes.len();
        let mut idx = util::first_not_of(attributes, b' ', 0);
        while idx < len {
            if attributes[idx] == b'>' {
                *str_all = str::from_utf8(&attributes[idx + 1..]).unwrap();
                return Some(res)
            }
            idx = res.next_attribute(attributes, idx)?;
        }
        None
    }
    fn next_attribute(&mut self, slice: &[u8], from: usize) -> Option<usize> {
        let len = slice.len();
        let eq = util::first_of(slice, b'=', from);
        let key = str::from_utf8(&slice[from..eq]).unwrap();
        if eq == len {
            eprintln!("[ERROR] There is no separator of an attribute for key: {}", key);
            return None
        }
        if eq + 1 == len {
            eprintln!("[ERROR] There is no value of an attribute for key: {}", key);
            return None
        }
        let to = self.end_of_value(slice, eq + 1, key)?;
        let value = str::from_utf8(&slice[eq + 1..to]).unwrap();
        self.push_attribute(String::from(key), String::from(value));
        Some(util::first_not_of(slice, b' ', to))
    }
    fn end_of_value(&mut self, slice: &[u8], from: usize, key: &str) -> Option<usize> {
        let len = slice.len();
        if slice[from] == b'"' {
            let dq = util::first_of(slice, b'"', from + 1);
            if dq == len {
                eprintln!("[ERROR] There is no value of an attribute for key: {}", key);
                return None
            }
            Some(dq + 1)
        } else if slice[from] == b' ' {
            eprintln!("[ERROR] There is no value of an attribute for key: {}", key);
            None
        } else {
            Some(util::first_of(slice, b' ', from))
        }
    }
    fn push_attribute(&mut self, key: String, value: String) {
        if !self.attributes.contains(&key) {
            self.attributes.push_unique(key, String::from(value));
        } else {
            eprintln!("[WARN] Duplicate attribute key found: {}", key);
        }
    }
    pub fn tag(&self) -> &str {
        &self.tag[..]
    }
    pub fn consume(&mut self, key: &str) -> Option<String> {
        let mut dest = String::new();
        mem::swap(self.attributes.get_mut(key)?, &mut dest);
        Some(dest)
    }
}

impl Display for Tag {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = self.tag.to_string();
        self.attributes.for_each(|key, value| {
            buf += " ";
            buf += key;
            buf += "=";
            buf += value;
        });
        write!(fmt, "{}", buf)
    }
}
