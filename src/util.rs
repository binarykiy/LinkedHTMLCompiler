use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ParsedText<'a> {
    Text(&'a str),
    Comment(&'a str),
    Tag(ParsedTag<'a>),
    CustomTag(ParsedTag<'a>),
    DocType(&'a str),
    Pointer(Box<ParsedText<'a>>),
}

impl<'a> ParsedText<'a> {
    pub fn parse(raw: &'a str) -> Option<Vec<Self>> {
        let mut res = Vec::new();
        let mut target = raw;
        while let Some(next_tag) = Self::next_tag(target, &mut res) {
            // Custom Tag Prefix (Comment Tag Prefix + '?')
            if next_tag.starts_with("!--?") {
                let next_tag = next_tag.split_once("!--?").unwrap().1;
                if let Some((raw_tag, other)) = next_tag.split_once("-->") {
                    if let Some(tag) = ParsedTag::new(raw_tag) {
                        res.push(Self::CustomTag(tag));
                        target = other;
                        continue;
                    } else {
                        // It is not necessary to throw an error to stop compilation
                        // because an incorrect custom tag is also a correct comment.
                        eprintln!("[WARN] The syntax of a custom tag is incorrect:");
                        eprintln!("\t{}", raw_tag);
                        // return None;
                    }
                } else {
                    eprintln!("[ERROR] There is no '-->' for a '<!--'.");
                    return None;
                }
            }
            // Comment Tag Prefix
            if next_tag.starts_with("!--") {
                let next_tag = next_tag.split_once("!--").unwrap().1;
                if let Some((comment, other)) = next_tag.split_once("-->") {
                    res.push(Self::Comment(comment));
                    target = other;
                    continue;
                } else {
                    eprintln!("[ERROR] There is no '-->' for a '<!--'.");
                    return None;
                }
            }
            // DOCTYPE Tag Prefix
            if next_tag.starts_with('!') {
                let next_tag = next_tag.split_once("!").unwrap().1;
                if let Some((doc_type, other)) = next_tag.split_once('>') {
                    res.push(Self::DocType(doc_type));
                    target = other;
                    continue;
                } else {
                    eprintln!("[ERROR] There is no '>' for a '<!'.");
                    return None;
                }
            }
            // Normal Tag
            if let Some((raw_tag, other)) = next_tag.split_once('>') {
                if let Some(tag) = ParsedTag::new(raw_tag) {
                    res.push(Self::Tag(tag));
                    target = other;
                    continue;
                } else {
                    eprintln!("[WARN] The syntax of a tag is incorrect:");
                    eprintln!("\t{}", raw_tag);
                    return None;
                }
            } else {
                eprintln!("[ERROR] There is no '>' for a '<'.");
                return None;
            }
        }
        Some(res)
    }
    fn next_tag(target: &'a str, dest: &mut Vec<Self>) -> Option<&'a str> {
        if let Some((text, other)) = target.split_once('<') {
            if !text.is_empty() {
                dest.push(Self::Text(text));
            }
            Some(other)
        } else {
            dest.push(Self::Text(target));
            None
        }
    }
}

impl<'a> Display for ParsedText<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
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
                write!(fmt, "{}", v)
            }
        }
    }
}

#[derive(Debug)]
pub struct ParsedTag<'a> {
    tag: &'a str,
    attributes: HashMap<&'a str, &'a str>,
}

impl<'a> ParsedTag<'a> {
    pub fn new(str_inside: &'a str) -> Option<Self> {
        let (tag, raw_attr) = str_inside.split_once(' ')
            .unwrap_or((str_inside, ""));
        let mut res = Self {
            tag,
            attributes: HashMap::new(),
        };
        if res.parse_raw_attr(raw_attr) {
            Some(res)
        } else {
            None
        }
    }
    fn parse_raw_attr(&mut self, raw_attr: &'a str) -> bool {
        for attribute in raw_attr.split(' ') {
            if let Some((key, value)) = attribute.split_once('=') {
                if !self.attributes.contains_key(&key) {
                    self.attributes.insert(key, value);
                } else {
                    eprintln!("[WARN] Duplicate attribute key found: {}", key);
                }
            } else {
                eprintln!("[ERROR] There is no separator of a key-value attribute pair");
                return false;
            }
        }
        true
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

impl<'a> Display for ParsedTag<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = self.tag.to_string();
        for (key, value) in &self.attributes {
            buf += " ";
            buf += key;
            buf += "=";
            buf += value;
        }
        write!(fmt, "{}", buf)
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
