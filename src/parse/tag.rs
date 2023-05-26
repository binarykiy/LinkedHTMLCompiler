use std::fmt::{Display, Formatter};
use std::{fmt, mem};

#[derive(Debug)]
pub struct Tag {
    tag: String,
    attr_key: Vec<String>,
    attr_value: Vec<String>,
}

impl Tag {
    pub fn new(str_inside: &str) -> Option<Self> {
        let (tag, raw_attr) = str_inside.split_once(' ')
            .unwrap_or((str_inside, ""));
        let mut res = Self {
            tag: String::from(tag),
            attr_key: Vec::new(),
            attr_value: Vec::new(),
        };
        if res.parse_attributes(raw_attr) {
            Some(res)
        } else {
            None
        }
    }
    fn parse_attributes(&mut self, raw_attr: &str) -> bool {
        // todo: avoid " " separating
        for attribute in raw_attr.split(' ') {
            if attribute.is_empty() {
                continue
            }
            if let Some((key, value)) = Self::parse_single_attr(attribute) {
                if !self.attr_key.contains(&key) {
                    self.attr_key.push(key);
                    self.attr_value.push(String::from(value));
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
    fn parse_single_attr(attribute: &str) -> Option<(String, String)> {
        if let Some((key, value)) = attribute.split_once('=') {
            if !key.is_empty() && !value.is_empty() {
                Some((String::from(key), String::from(value)))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn tag(&self) -> &str {
        &self.tag[..]
    }
    pub fn consume<F: FnOnce(String)>(&mut self, key: &str, consumer: F) {
        self.consume_or(key, consumer, || {});
    }
    pub fn consume_or<F: FnOnce(String), G: FnOnce()>(&mut self, key: &str, consumer: F, or: G) {
        match self.move_value(key) {
            Some(v) => consumer(v),
            None => or(),
        }
    }
    pub fn move_value(&mut self, key: &str) -> Option<String> {
        for i in 0..self.attr_key.len() {
            if self.attr_key[i] == key {
                let mut dest = String::new();
                mem::swap(&mut self.attr_value[i], &mut dest);
                return Some(dest);
            }
        }
        None
    }
    pub fn clean(self) {
        for key in self.attr_key {
            eprintln!("[WARN] Attribute '{}' does not work in Tag '{}'", key, self.tag);
        }
    }
}

impl Display for Tag {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = self.tag.to_string();
        for i in 0..self.attr_key.len() {
            buf += " ";
            buf += self.attr_key[i].as_str();
            buf += "=";
            buf += self.attr_value[i].as_str();
        }
        write!(fmt, "{}", buf)
    }
}
