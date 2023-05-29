mod include;

use once_cell::sync::Lazy;
use crate::config::Config;
use crate::parse::doc::Doc;
use crate::parse::tag::Tag;
use crate::util::VecDict;

static CUSTOM_TAGS: Lazy<VecDict<&'static str, fn(Tag, &mut Config) -> Option<Doc>>> = Lazy::new(|| {
    let mut dict: VecDict<&'static str, fn(Tag, &mut Config) -> Option<Doc>> = VecDict::new();
    dict.push_unique("include", include::run);
    dict
});

pub fn run(tag: Tag, config: &mut Config) -> Option<Doc> {
    let func = CUSTOM_TAGS.get(&tag.tag())?;
    func(tag, config)
}
