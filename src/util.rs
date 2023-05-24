use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ParsedTag<'a> {
    is_original: bool,
    tag: &'a str,
    attributes: HashMap<&'a str, &'a str>,
}

impl<'a> ParsedTag<'a> {
    pub fn new(str_inside: &'a str) -> Option<Self> {
        let (mut tag, mut raw_attr) = str_inside.split_once(' ')
            .unwrap_or((str_inside, ""));
        let mut is_original = false;
        if tag.starts_with("!--?") {
            is_original = true;
            tag = tag.split_once("!--?").unwrap().1;
            if raw_attr.is_empty() {
                tag = tag.rsplit_once("--")
                    .expect("Illegal Syntax").0;
            } else {
                raw_attr = raw_attr.rsplit_once("--")
                    .expect("Illegal Syntax").1;
            }
        } else if tag.starts_with("!") {
            return None;
        }
        let mut res = Self {
            is_original,
            tag,
            attributes: HashMap::new(),
        };
        res.parse_raw_attr(raw_attr);
        Some(res)
    }
    fn parse_raw_attr(&mut self, raw_attr: &'a str) {
        for attribute in raw_attr.split(' ') {
            let (key, value) = attribute.split_once('=')
                .expect("Illegal Syntax");
            if !self.attributes.contains_key(&key) {
                self.attributes.insert(key, value);
            } else {
                eprintln!("[WARN] Duplicate attribute key found: {}", key);
            }
        }
    }
    pub fn is_original(&self) -> bool {
        self.is_original
    }
    pub fn tag(&self) -> &str {
        self.tag
    }
    pub fn consume<F: FnOnce(&'a str)>(&mut self, key: &str, consumer: F) {
        self.consume_or(key, consumer, || {});
    }
    pub fn consume_or<F: FnOnce(&'a str), G: FnOnce()>(&mut self, key: &str, consumer: F, or: G) {
        match self.attributes.remove(key) {
            Some(v) => consumer(v),
            None => or(),
        }
    }
    pub fn clean(self) {
        for attr in self.attributes {
            eprintln!("[WARN] Attribute '{}' does not work in Tag '{}'", attr.0, self.tag);
        }
    }
}

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
