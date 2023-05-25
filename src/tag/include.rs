use crate::config::Config;
use crate::{parse_and_write, read_all};
use crate::util::{extract, ParsedTag, ParsedText, SyntaxError};

pub fn run<'a>(mut source: ParsedTag<'a>, config: &mut Config) -> Option<Vec<ParsedText<'a>>> {
    source.consume("link", |value| {
        let link = value.trim_matches('"');
        let source = read_all(config.relative_path(link))
            .expect(format!("[ERROR] Failed to read the linked file: {}", value).as_str());
        // todo: use parse()
        match extract(source.as_str(), "body") {
            Ok(body) => {
                parse_and_write(body.to_string(), config);
            }
            Err(e) => {
                if let SyntaxError::NoTagBegin(_) = e {
                    parse_and_write(source, config);
                } else {
                    eprintln!("[FATAL] {}", e);
                    panic!()
                }
            }
        }
    });
    None
}
