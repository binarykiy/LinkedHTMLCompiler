use std::fmt::{Display, Formatter};
use std::fmt;
use crate::parse::tag::Tag;

#[derive(Debug)]
pub enum Token {
    Text(String),
    Comment(String),
    Tag(Tag),
    CustomTag(Tag),
    DocType(String),
    Pointer(Vec<Token>),
    Null,
}

impl Token {
    pub fn parse(raw: &str) -> Option<Vec<Self>> {
        let mut res = Vec::new();
        let mut target = raw;
        while let Some(next_tag) = Self::next_tag(target, &mut res) {
            // Custom Tag Prefix (Comment Tag Prefix + '?')
            if next_tag.starts_with("!--?") {
                let next_tag = next_tag.split_once("!--?").unwrap().1;
                if let Some((raw_tag, other)) = next_tag.split_once("-->") {
                    if let Some(tag) = Tag::new(raw_tag) {
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
                if let Some(tag) = Tag::new(raw_tag) {
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
