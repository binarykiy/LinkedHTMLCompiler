use std::collections::VecDeque;
use crate::config::Config;
use crate::{parse, read_all};
use crate::parse::token::Token;
use crate::parse::tag::Tag;

pub fn run(mut source: Tag, config: &mut Config) -> Option<Vec<Token>> {
    let mut res = None;
    source.consume("link", |value| {
        let link = value.trim_matches('"');
        let source = read_all(config.relative_path(link))
            .expect(format!("[ERROR] Failed to read the linked file: {}", value).as_str());
        let parsed = parse(source.as_str(), config);
        let len = parsed.len();
        let mut begin = len;
        let mut end = len;
        for i in 0..len {
            if let Token::Tag(tag) = &parsed[i] {
                if tag.tag() == "body" {
                    if begin == len {
                        begin = i;
                    } else {
                        eprintln!("[ERROR] Duplicate <body> tags found");
                        return;
                    }
                }
                if tag.tag() == "/body" {
                    if end == len {
                        end = i;
                    } else {
                        eprintln!("[ERROR] Duplicate </body> tags found");
                        return;
                    }
                }
            }
        }
        if begin == len && end == len {
            res = Some(parsed);
            return;
        }
        if begin != len && end != len && begin < end {
            let mut tmp = VecDeque::from(parsed);
            for _ in 0..=begin {
                tmp.pop_front();
            }
            for _ in end..len {
                tmp.pop_back();
            }
            res = Some(Vec::from(tmp));
            return;
        }
    });
    res
}
