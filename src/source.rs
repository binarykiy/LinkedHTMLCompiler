pub struct SourceManager<'a> {
    source: &'a [u8],
    begin: usize,
}

impl<'a> SourceManager<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source.as_bytes(),
            begin: 0,
        }
    }
}
