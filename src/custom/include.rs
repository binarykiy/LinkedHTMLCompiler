use crate::config::Config;
use crate::{parse, read_all};
use crate::parse::doc::Doc;
use crate::parse::tag::Tag;

pub fn run(mut source: Tag, config: &mut Config) -> Option<Doc> {
    let mut res = None;
    source.consume("link", |value| {
        let link = value.trim_matches('"');
        let source = read_all(config.relative_path(link))
            .expect(format!("[ERROR] Failed to read the linked file: {}", value).as_str());
        let mut parsed = parse(source.as_str(), config);
        let len = parsed.len();
        let mut begin = len;
        let mut end = len;
        let mut err = false;
        parsed.find_tag(|i, tag| {
            if tag.tag() == "body" {
                if begin == len {
                    begin = i;
                } else {
                    eprintln!("[ERROR] Duplicate <body> tags found");
                    err = true;
                    return;
                }
            }
            if tag.tag() == "/body" {
                if end == len {
                    end = i;
                } else {
                    eprintln!("[ERROR] Duplicate </body> tags found");
                    err = true;
                    return;
                }
            }
        });
        if err {
            return;
        }
        if begin == len && end == len {
            res = Some(parsed);
            return;
        }
        if begin != len && end != len {
            parsed.extract(begin+1..end);
            res = Some(parsed);
            return;
        }
    });
    res
}
