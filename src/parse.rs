use std::process;
use std::rc::Rc;
use crate::config::Config;
use crate::custom;
use crate::parse::doc::Doc;
use crate::parse::component::Component;

pub mod tag;
pub mod component;
pub mod doc;

pub fn into_doc(source: Rc<String>, cfg: &mut Config) -> Doc {
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
