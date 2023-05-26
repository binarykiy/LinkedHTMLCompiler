use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut, RangeBounds};
use crate::parse::tag::Tag;
use crate::parse::token::Token;

#[derive(Debug)]
pub struct Doc {
    doc: VecDeque<Token>,
}

impl Doc {
    pub fn parse(doc: String) -> Option<Self> {
        let mut res = Vec::new();
        let mut target = doc.as_str();
        while let Some(next_tag) = Self::next_tag(target, &mut res) {
            // Custom Tag Prefix (Comment Tag Prefix + '?')
            if next_tag.starts_with("!--?") {
                let next_tag = next_tag.split_once("!--?").unwrap().1;
                if let Some((raw_tag, other)) = next_tag.split_once("-->") {
                    if let Some(tag) = Tag::new(raw_tag) {
                        res.push(Token::CustomTag(tag));
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
                    res.push(Token::Comment(String::from(comment)));
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
                    res.push(Token::DocType(String::from(doc_type)));
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
                    res.push(Token::Tag(tag));
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
        Some(Self {
            doc: res.into(),
        })
    }
    fn next_tag<'a, 'b>(target: &'a str, dest: &'b mut Vec<Token>) -> Option<&'a str> {
        if let Some((text, other)) = target.split_once('<') {
            if !text.is_empty() {
                dest.push(Token::Text(String::from(text)));
            }
            Some(other)
        } else {
            dest.push(Token::Text(String::from(target)));
            None
        }
    }
    pub fn extract<R: RangeBounds<usize>>(&mut self, range: R) {
        let mut triggered = false;
        let len = self.doc.len();
        for i in 0..len {
            if !range.contains(&i) {
                if triggered {
                    self.doc.pop_back();
                } else {
                    self.doc.pop_front();
                }
            } else {
                triggered = true;
            }
        }
    }
    pub fn find_tags(&self, tag_name: &str) -> Vec<usize> {
        let mut vec = Vec::new();
        let len = self.doc.len();
        for i in 0..len {
            let Token::Tag(tag) = &self[i] else { continue };
            if tag.tag() == tag_name {
                vec.push(i);
            }
        }
        vec
    }
    pub fn reassign_custom<F: FnMut(Tag) -> Token>(&mut self, mut func: F) {
        let len = self.doc.len();
        for i in 0..len {
            if let Token::CustomTag(_) = &self[i] {
                let Token::CustomTag(tag) = self[i].swap_null() else { unreachable!() };
                self[i] = func(tag);
            }
        }
    }
}

impl Index<usize> for Doc {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        self.doc.index(index)
    }
}

impl IndexMut<usize> for Doc {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.doc.index_mut(index)
    }
}

impl Display for Doc {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        for token in &self.doc {
            write!(fmt, "{}", token)?;
        }
        Ok(())
    }
}
