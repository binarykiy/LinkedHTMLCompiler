use std::{io, mem};
use std::io::{BufRead, BufReader};

pub fn read_from_stdin() -> io::Result<String> {
    let mut buf = String::new();
    BufReader::new(io::stdin().lock()).read_line(&mut buf)?;
    Ok(buf)
}

pub struct Lazy<K, T> {
    func: fn(K) -> T,
    key: Option<K>,
    inner: Option<T>,
}

impl<K, T> Lazy<K, T> {
    pub fn new(func: fn(K) -> T, key: K) -> Self {
        Self {
            func,
            key: Some(key),
            inner: None,
        }
    }
    pub fn get_mut(&mut self) -> &mut T {
        if let None = self.inner {
            let key = self.get_key();
            self.inner = Some((self.func)(key));
        }
        let Some(inner) = &mut self.inner else { unreachable!() };
        inner
    }
    fn get_key(&mut self) -> K {
        mem::replace(&mut self.key, None).unwrap()
    }
}

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
    pub fn contains<T: PartialEq<K> + ?Sized>(&self, key: &T) -> bool {
        let len = self.dict.len();
        for i in 0..len {
            if *key == self.dict[i].0 {
                return true
            }
        }
        false
    }
    pub fn get<T: PartialEq<K> + ?Sized>(&self, key: &T) -> Option<&V> {
        let len = self.dict.len();
        for i in 0..len {
            if *key == self.dict[i].0 {
                return Some(&self.dict[i].1)
            }
        }
        None
    }
    pub fn get_mut<T: PartialEq<K> + ?Sized>(&mut self, key: &T) -> Option<&mut V> {
        let len = self.dict.len();
        for i in 0..len {
            if *key == self.dict[i].0 {
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
