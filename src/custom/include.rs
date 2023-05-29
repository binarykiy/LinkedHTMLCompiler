use crate::config::Config;
use crate::parse;
use crate::parse::doc::Doc;
use crate::parse::tag::Tag;

pub fn run(mut tag: Tag, cfg: &mut Config) -> Option<Doc> {
    let link_raw = tag.consume("link")?;
    let link = link_raw.trim_matches('"');
    let source = cfg.read_relative(link)
        .expect(format!("[ERROR] Failed to read the linked file: {}", link).as_str());
    let mut linked_doc = parse::into_doc(source, cfg);
    let begin = linked_doc.find_tags("body");
    let end = linked_doc.find_tags("/body");
    validate_body_tag(&begin, &end);
    if begin.len() == 1 && begin.len() == 1 {
        linked_doc.extract(begin[0]+1..end[0]);
    }
    tag.clean();
    Some(linked_doc)
}

fn validate_body_tag(begin: &Vec<usize>, end: &Vec<usize>) -> bool {
    if begin.len() > 1 {
        eprintln!("[ERROR] Duplicate <body> tags found");
        return false
    }
    if end.len() > 1 {
        eprintln!("[ERROR] Duplicate </body> tags found");
        return false
    }
    if begin.is_empty() && !end.is_empty() {
        eprintln!("[ERROR] Did not found <body> for </body>");
        return false
    }
    if !begin.is_empty() && end.is_empty() {
        eprintln!("[ERROR] Did not found </body> for <body>");
        return false
    }
    true
}
