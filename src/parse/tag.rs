use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct Tag {
    tag: String,
    attributes: HashMap<String, String>,
}

impl Tag {
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

impl Display for Tag {
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
