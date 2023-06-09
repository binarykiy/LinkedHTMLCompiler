use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut, RangeBounds};
use std::rc::Rc;
use std::str;
use crate::parse::tag::{BinaryTag, Tag};
use crate::parse::component::{BinaryComponent, Component};
use crate::source::SourceManager;
use crate::util;

#[derive(Debug)]
pub struct BinaryDoc {
    doc: VecDeque<BinaryComponent>,
}

impl BinaryDoc {
    pub fn new(source: Rc<String>) -> Option<Self> {
        let mut doc = Self {
            doc: VecDeque::new(),
        };
        let mut source = SourceManager::new(&*source);
        while doc.push_text_and_next(&mut source) {
            if source.pop_if_starts_with(b"!--?") {
                doc.push_custom_tag(&mut source)?;
            } else if source.pop_if_starts_with(b"!--") {
                doc.push_comment(&mut source)?;
            } else if source.pop_if_starts_with(b"!") {
                doc.push_doc_type(&mut source)?;
            } else {
                doc.push_tag(&mut source)?;
            }
        }
        Some(doc)
    }
    fn push_text_and_next(&mut self, source: &mut SourceManager) -> bool {
        let res = source.next_at_first_of(b"<");
        let text = source.partially_to_vec();
        if !text.is_empty() {
            self.push(BinaryComponent::Text(text));
        }
        if res {
            source.move_to_next();
        }
        res
    }
    fn push_custom_tag(&mut self, source: &mut SourceManager) -> Option<()> {
        if !source.next_at_first_of(b"-->") {
            return None
        }
        if let Some(tag) = BinaryTag::new_custom(source.partially_from()) {
            self.push(BinaryComponent::CustomTag(tag));
        } else {
            self.push(BinaryComponent::CustomComment(source.partially_to_vec()));
        }
        source.move_to_next();
        Some(())
    }
    fn push_comment(&mut self, source: &mut SourceManager) -> Option<()> {
        if !source.next_at_first_of(b"-->") {
            return None
        }
        let comment = source.partially_to_vec();
        self.push(BinaryComponent::Comment(comment));
        source.move_to_next();
        Some(())
    }
    fn push_doc_type(&mut self, source: &mut SourceManager) -> Option<()> {
        if !source.next_at_first_of(b">") {
            return None
        }
        let doc_type = source.partially_to_vec();
        self.push(BinaryComponent::DocType(doc_type));
        source.move_to_next();
        Some(())
    }
    fn push_tag(&mut self, source: &mut SourceManager) -> Option<()> {
        if !source.next_at_first_of(b">") {
            return None
        }
        let tag = BinaryTag::new(source)?;
        self.push(BinaryComponent::Tag(tag));
        Some(())
    }
    fn push(&mut self, component: BinaryComponent) {
        self.doc.push_back(component);
    }
}

#[derive(Debug)]
pub struct Doc {
    doc: VecDeque<Component>,
}

impl Doc {
    pub fn new(doc: Rc<String>) -> Option<Self> {
        let mut res = Vec::new();
        let mut target = doc.as_str();
        while let Some(next_tag) = Self::skip_text(target, &mut res) {
            target = next_tag;
            if next_tag.starts_with("<!--") {
                let comment = Self::parse_comment(&mut target)?;
                if let Component::CustomComment(tag_source) = comment {
                    res.push(Self::parse_custom_tag(tag_source));
                }
                continue
            }
            if next_tag.starts_with("<!") {
                let doc_type = Self::parse_doc_type(&mut target)?;
                res.push(doc_type);
                continue
            }
            if next_tag.starts_with("<") {
                let tag = Self::parse_tag(&mut target)?;
                res.push(tag);
                continue
            }
        }
        Some(Self {
            doc: res.into(),
        })
    }
    fn skip_text<'a, 'b>(target: &'a str, dest: &'b mut Vec<Component>) -> Option<&'a str> {
        let bytes = target.as_bytes();
        let idx = util::first_of(bytes, b'<', 0);
        if idx == bytes.len() {
            let text = String::from(target);
            dest.push(Component::Text(text));
            return None
        }
        if idx > 0 {
            let text = String::from(str::from_utf8(&bytes[..idx]).unwrap());
            dest.push(Component::Text(text));
        }
        Some(str::from_utf8(&bytes[idx..]).unwrap())
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
            Some(tag) => Component::CustomTag(tag, content),
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
            if let Component::CustomTag(_, _) = &self[i] {
                let Component::CustomTag(tag, _) = self[i].swap_null() else { unreachable!() };
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
