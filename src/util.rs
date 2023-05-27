#[derive(Debug)]
pub struct VecDict<K: PartialEq, V> {
    dict: Vec<(K, V)>,
}

impl<K: PartialEq, V> VecDict<K, V> {
    pub fn new() -> Self {
        Self {
            dict: Vec::new(),
        }
    }
    pub fn push_unique(&mut self, key: K, value: V) {
        if !self.contains(&key) {
            self.dict.push((key, value));
        }
    }
    pub fn contains(&self, key: &K) -> bool {
        let len = self.dict.len();
        for i in 0..len {
            if self.dict[i].0 == *key {
                return true
            }
        }
        false
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let len = self.dict.len();
        for i in 0..len {
            if self.dict[i].0 == *key {
                return Some(&mut self.dict[i].1)
            }
        }
        None
    }
    pub fn for_each<F: FnMut(&K, &V)>(&self, mut func: F) {
        let len = self.dict.len();
        for i in 0..len {
            let (k, v) = &self.dict[i];
            func(k, v);
        }
    }
}
