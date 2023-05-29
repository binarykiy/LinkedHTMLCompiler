use std::{io, mem};
use std::io::{BufRead, BufReader};

pub fn first_of(slice: &[u8], target: u8, from: usize) -> usize {
    let len = slice.len();
    for i in from..len {
        if slice[i] == target {
            return i
        }
    }
    len
}

pub fn first_not_of(slice: &[u8], target: u8, from: usize) -> usize {
    let len = slice.len();
    for i in from..len {
        if slice[i] != target {
            return i
        }
    }
    len
}

pub fn read_from_stdin() -> io::Result<String> {
    let mut buf = String::new();
    BufReader::new(io::stdin().lock()).read_line(&mut buf)?;
    Ok(buf)
}

pub struct LazyClosure<K, T> {
    inner: LazyStatus<K, T>,
}

impl<K, T> LazyClosure<K, T> {
    pub const fn new(func: fn(K) -> T, key: K) -> Self {
        Self {
            inner: LazyStatus::Uninitialized(func, Some(key)),
        }
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.init();
        let LazyStatus::Initialized(res) = &mut self.inner
            else { unreachable!() };
        res
    }
}

enum LazyStatus<K, T> {
    Uninitialized(fn(K) -> T, Option<K>),
    Initialized(T),
}

impl<K, T> LazyStatus<K, T> {
    fn init(&mut self) {
        let Self::Uninitialized(func, key) = self
            else { return };
        let key = mem::replace(key, None).unwrap();
        let inner = func(key);
        *self = Self::Initialized(inner);
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
