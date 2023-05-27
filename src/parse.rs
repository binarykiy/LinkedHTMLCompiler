use std::process;
use std::rc::Rc;
use crate::config::Config;
use crate::custom;
use crate::parse::doc::Doc;
use crate::parse::tag::Tag;
use crate::parse::token::Token;

pub mod tag;
pub mod token;
pub mod doc;

pub fn parse(source: Rc<String>, cfg: &mut Config) -> Doc {
    let mut doc = Doc::new(source).unwrap_or_else(|| {
        process::exit(0);
    });
    doc.reassign_custom(|tag| {
        if let Some(v) = compile(tag, cfg) {
            Token::Pointer(v)
        } else {
            Token::Comment(String::from("?error"))
        }
    });
    doc
}

fn compile(tag: Tag, cfg: &mut Config) -> Option<Doc> {
    match tag.tag() {
        "include" => {
            custom::include(tag, cfg)
        }
        _ => {
            None
        }
    }
}
