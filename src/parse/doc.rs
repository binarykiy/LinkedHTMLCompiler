use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut, RangeBounds};
use std::rc::Rc;
use std::str;
use crate::parse::tag::Tag;
use crate::parse::component::Component;

#[derive(Debug)]
pub struct Doc {
    doc: VecDeque<Component>,
}

impl Doc {
    pub fn new(doc: Rc<String>) -> Option<Self> {
        let mut res = Vec::new();
        let mut target = doc.as_str();
        while let Some(next_tag) = Self::skip_text(target, &mut res) {
            // Custom Tag Prefix (Comment Tag Prefix + '?')
            if next_tag.starts_with("!--?") {
                let next_tag = next_tag.split_once("!--?").unwrap().1;
                if let Some((raw_tag, other)) = next_tag.split_once("-->") {
                    if let Some(tag) = Tag::new(raw_tag) {
                        res.push(Component::CustomTag(tag));
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
                    res.push(Component::Comment(String::from(comment)));
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
                    res.push(Component::DocType(String::from(doc_type)));
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
                    res.push(Component::Tag(tag));
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
    fn next_tag<'a, 'b>(target: &'a str, dest: &'b mut Vec<Component>) -> Option<&'a str> {
        if let Some((text, other)) = target.split_once('<') {
            if !text.is_empty() {
                dest.push(Component::Text(String::from(text)));
            }
            Some(other)
        } else {
            dest.push(Component::Text(String::from(target)));
            None
        }
    }
    fn skip_text<'a, 'b>(target: &'a str, dest: &'b mut Vec<Component>) -> Option<&'a str> {
        let bytes = target.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == b'<' {
                if i > 0 {
                    let text = String::from(str::from_utf8(&bytes[..i]).unwrap());
                    dest.push(Component::Text(text));
                }
                return Some(str::from_utf8(&bytes[i..]).unwrap())
            }
        }
        let text = String::from(target);
        dest.push(Component::Text(text));
        None
    }
    fn parse_comment(target: &mut &str) -> Option<Component> {
        debug_assert!(target.starts_with("<!--"));
        let Some((_, other)) = target.split_once("<!--")
            else { return None };
        let Some((content, other)) = other.split_once("-->")
            else { return None };
        *target = other;
        Some(Self::create_comment(content))
    }
    fn create_comment(content: &str) -> Component {
        if content.starts_with("?") {
            let Some((_, value)) = content.split_once("?")
                else { unreachable!() };
            let value_owned = String::from(value);
            Component::CustomComment(value_owned)
        } else {
            let value_owned = String::from(content);
            Component::Comment(value_owned)
        }
    }
    fn parse_custom_tag(content: String) -> Component {
        match Tag::new(content.as_str()) {
            Some(tag) => Component::CustomTagNew(tag, content),
            None => Component::CustomComment(content),
        }
    }
    fn parse_doc_type(target: &mut &str) -> Option<Component> {
        debug_assert!(target.starts_with("<!"));
        let Some((_, other)) = target.split_once("<!")
            else { return None };
        let Some((content, other)) = other.split_once(">")
            else { return None };
        *target = other;
        Some(Component::DocType(String::from(content)))
    }
    fn parse_tag(target: &mut &str) -> Option<Component> {
        debug_assert!(target.starts_with("<"));
        let tag = Tag::new_once(target)?;
        Some(Component::Tag(tag))
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
            let Component::Tag(tag) = &self[i] else { continue };
            if tag.tag() == tag_name {
                vec.push(i);
            }
        }
        vec
    }
    pub fn reassign_custom<F: FnMut(Tag) -> Component>(&mut self, mut func: F) {
        let len = self.doc.len();
        for i in 0..len {
            if let Component::CustomTag(_) = &self[i] {
                let Component::CustomTag(tag) = self[i].swap_null() else { unreachable!() };
                self[i] = func(tag);
            }
        }
    }
}

impl Index<usize> for Doc {
    type Output = Component;

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
