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
    pub fn find_first_of(&self, bytes: &[u8]) -> usize {
        let len = bytes.len();
        let to = self.source.len() - len;
        for i in self.begin..=to {
            if &self.source[i..=i + len] == bytes {
                return i
            }
        }
        to + 1
    }
    pub fn find_first_not_of(&self, bytes: &[u8]) -> usize {
        let len = bytes.len();
        let to = self.source.len() - len;
        for i in self.begin..=to {
            if &self.source[i..=i + len] != bytes {
                return i
            }
        }
        to + 1
    }
    pub fn retain_from(&mut self, from: usize) {
        self.validate_index(from);
        self.begin = from;
    }
    fn validate_index(&self, idx: usize) {
        if idx > self.source.len() {
            panic!("SourceManager: byte index {idx} is out of bounds")
        }
    }
}
