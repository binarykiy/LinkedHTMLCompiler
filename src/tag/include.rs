use crate::config::Config;
use crate::{parse_and_write, read_all};

pub fn run<'a, F: Iterator<Item=(&'a str, &'a str)>>(iter: F, config: &mut Config) {
    for (key, value) in iter {
        match key {
            "link" => {
                let link = value.trim_matches('"');
                let mut source = read_all(config.relative_path(link))
                    .expect(format!("[ERROR] Failed to read the linked file: {}", value).as_str());
                if let Some((_, body_from_prefix)) = source.split_once("<body") {
                    let (_, body_after_prefix) = body_from_prefix.split_once('>')
                        .expect("[ERROR] Expected '>', not found");
                    let (body, _) = body_after_prefix.split_once("</body>")
                        .expect("[ERROR] Expected '</body>', not found");
                    source = body.to_string();
                }
                parse_and_write(source, config);
            }
            _ => {
                eprintln!("[WARN] Unknown property for \"include\": \"{}={}\"", key, value);
            }
        }
    }
}
