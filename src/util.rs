use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ParsedText {
    Text(String),
    Comment(String),
    Tag(ParsedTag),
    CustomTag(ParsedTag),
    DocType(String),
    Pointer(Vec<ParsedText>),
    Null,
}

impl ParsedText {
    pub fn parse(raw: &str) -> Option<Vec<Self>> {
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
                    res.push(Self::Comment(String::from(comment)));
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
                    res.push(Self::DocType(String::from(doc_type)));
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
    fn next_tag<'a, 'b>(target: &'a str, dest: &'b mut Vec<Self>) -> Option<&'a str> {
        if let Some((text, other)) = target.split_once('<') {
            if !text.is_empty() {
                dest.push(Self::Text(String::from(text)));
            }
            Some(other)
        } else {
            dest.push(Self::Text(String::from(target)));
            None
        }
    }
}

impl Display for ParsedText {
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
                for text in v {
                    write!(fmt, "{}", text)?;
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

#[derive(Debug)]
pub struct ParsedTag {
    tag: String,
    attributes: HashMap<String, String>,
}

impl ParsedTag {
    pub fn new(str_inside: &str) -> Option<Self> {
        let (tag, raw_attr) = str_inside.split_once(' ')
            .unwrap_or((str_inside, ""));
        let mut res = Self {
            tag: String::from(tag),
            attributes: HashMap::new(),
        };
        if res.parse_raw_attr(raw_attr) {
            Some(res)
        } else {
            None
        }
    }
    fn parse_raw_attr(&mut self, raw_attr: &str) -> bool {
        for attribute in raw_attr.split(' ') {
            if let Some((key, value)) = attribute.split_once('=') {
                if !self.attributes.contains_key(key) {
                    self.attributes.insert(String::from(key), String::from(value));
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
        &self.tag[..]
    }
    pub fn consume<F: FnOnce(String)>(&mut self, key: &str, consumer: F) {
        self.consume_or(key, consumer, || {});
    }
    pub fn consume_or<F: FnOnce(String), G: FnOnce()>(&mut self, key: &str, consumer: F, or: G) {
        match self.attributes.remove(key) {
            Some(v) => consumer(v),
            None => or(),
        }
    }
    pub fn clean(self) {
        for (key, _) in self.attributes {
            eprintln!("[WARN] Attribute '{}' does not work in Tag '{}'", key, self.tag);
        }
    }
}

impl Display for ParsedTag {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
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
