use std::fmt::{Display, Formatter};
use std::{fmt, mem, str};

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
        let attributes = raw_attr.as_bytes();
        let len = attributes.len();
        let mut idx = find_first_not_of(attributes, b' ', 0);
        while idx < len {
            idx = res.next_attribute(attributes, idx)?;
        }
        Some(res)
    }
    fn next_attribute(&mut self, slice: &[u8], from: usize) -> Option<usize> {
        let len = slice.len();
        let eq = find_first_of(slice, b'=', from);
        let key = str::from_utf8(&slice[from..eq]).unwrap();
        if eq == len {
            eprintln!("[ERROR] There is no separator of an attribute for key: {}", key);
            return None
        }
        if eq + 1 == len {
            eprintln!("[ERROR] There is no value of an attribute for key: {}", key);
            return None
        }
        let to;
        if slice[eq + 1] == b'"' {
            let dq = find_first_of(slice, b'"', eq + 2);
            if dq == len {
                eprintln!("[ERROR] There is no value of an attribute for key: {}", key);
                return None
            }
            to = dq + 1;
        } else if slice[eq + 1] == b' ' {
            eprintln!("[ERROR] There is no value of an attribute for key: {}", key);
            return None
        } else {
            to = find_first_of(slice, b' ', eq);
        }
        let value = str::from_utf8(&slice[eq + 1..to]).unwrap();
        self.push_attribute(String::from(key), String::from(value));
        Some(find_first_not_of(slice, b' ', to))
    }
    fn push_attribute(&mut self, key: String, value: String) {
        if !self.attr_key.contains(&key) {
            self.attr_key.push(key);
            self.attr_value.push(String::from(value));
        } else {
            eprintln!("[WARN] Duplicate attribute key found: {}", key);
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

fn find_first_of(slice: &[u8], target: u8, from: usize) -> usize {
    let len = slice.len();
    for i in from..len {
        if slice[i] == target {
            return i
        }
    }
    len
}

fn find_first_not_of(slice: &[u8], target: u8, from: usize) -> usize {
    let len = slice.len();
    for i in from..len {
        if slice[i] != target {
            return i
        }
    }
    len
}
