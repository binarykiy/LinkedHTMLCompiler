pub struct SourceManager<'a> {
    source: &'a [u8],
    from: usize,
    end: usize,
    next_from: usize,
}

impl<'a> SourceManager<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source.as_bytes(),
            from: 0,
            end: 0,
            next_from: 0,
        }
    }
    pub fn find_first_of(&self, bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        let to = self.end - len;
        for i in self.from..=to {
            if &self.source[i..=i + len] == bytes {
                return Some(i)
            }
        }
        None
    }
    pub fn find_first_not_of(&self, bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        let to = self.end - len;
        for i in self.from..=to {
            if &self.source[i..=i + len] != bytes {
                return Some(i)
            }
        }
        None
    }
    pub fn retain_from(&mut self, from: usize) {
        self.validate_index(from);
        self.from = from;
    }
    fn validate_index(&self, idx: usize) {
        if idx > self.source.len() {
            panic!("SourceManager: byte index {idx} is out of bounds")
        }
    }
    pub fn pop_if_starts_with(&mut self, bytes: &[u8]) -> bool {
        let len = bytes.len();
        if self.end - self.from < len {
            return false
        }
        for i in self.from..len {
            if self.source[i] != bytes[i] {
                return false
            }
        }
        self.from += len;
        true
    }
}
