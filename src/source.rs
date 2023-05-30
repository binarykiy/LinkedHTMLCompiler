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
    pub fn next_at_first_of(&mut self, bytes: &[u8]) -> bool {
        let len = bytes.len();
        let to = self.end - len;
        for i in self.from..=to {
            if &self.source[i..i + len] == bytes {
                self.end = i;
                self.next_from = i + len;
                return true
            }
        }
        false
    }
    pub fn next_at_first_not_of(&mut self, bytes: &[u8]) -> bool {
        let len = bytes.len();
        let to = self.end - len;
        for i in self.from..=to {
            if &self.source[i..i + len] != bytes {
                self.end = i;
                self.next_from = i + len;
                return true
            }
        }
        false
    }
    pub fn move_to_next(&mut self) {
        self.from = self.next_from;
        self.end = self.source.len();
        self.next_from = self.source.len();
    }
    pub fn is_empty(&self) -> bool {
        self.from == self.source.len()
    }
    fn validate_index(&self, idx: usize) {
        if idx < self.from || idx > self.end {
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
