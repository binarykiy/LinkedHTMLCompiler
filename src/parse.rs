use std::process;
use std::rc::Rc;
use crate::config::Config;
use crate::custom;
use crate::parse::doc::Doc;
use crate::parse::token::Component;

pub mod tag;
pub mod token;
pub mod doc;

pub fn parse(source: Rc<String>, cfg: &mut Config) -> Doc {
    let mut doc = Doc::new(source).unwrap_or_else(|| {
        process::exit(0);
    });
    doc.reassign_custom(|tag| {
        if let Some(v) = custom::run(tag, cfg) {
            Component::Pointer(v)
        } else {
            Component::Comment(String::from("?error"))
        }
    });
    doc
}
