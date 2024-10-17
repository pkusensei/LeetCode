mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct WordFilter {
    trie: Trie,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for (idx, w) in words.iter().enumerate() {
            let s = w.bytes().chain(std::iter::once(b'{')).chain(w.bytes());
            for len in 0..w.len() {
                trie.insert(s.clone().skip(len), idx as i32, index_of);
            }
        }
        Self { trie }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let s = suff
            .bytes()
            .chain(std::iter::once(b'{'))
            .chain(pref.bytes());
        self.trie.find(s, index_of).unwrap_or(-1)
    }
}

#[derive(Debug, Clone)]
struct Trie {
    data: [Option<Box<Trie>>; 27],
    num: Option<i32>,
}

impl Trie {
    pub const fn new() -> Self {
        Self {
            data: [const { None }; 27],
            num: None,
        }
    }

    pub fn insert<T, I, F>(&mut self, input: I, num: i32, index_of: F)
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.insert_impl(input.into_iter(), num, index_of);
    }

    fn insert_impl<T, I, F>(&mut self, mut it: I, num: i32, mut index_of: F)
    where
        I: Iterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.num = Some(num);
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it, num, index_of);
            } else {
                let mut node = Box::new(Self::new());
                node.insert(it, num, index_of);
                self.data[idx] = Some(node);
            }
        }
    }

    pub fn find<T, I, F>(&self, input: I, index_of: F) -> Option<i32>
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.find_impl(input.into_iter(), index_of)
    }

    fn find_impl<T, I, F>(&self, mut it: I, mut index_of: F) -> Option<i32>
    where
        I: Iterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(node) = self.data.get(idx).and_then(|n| n.as_ref()) {
                return node.find(it, index_of);
            } else {
                return None;
            }
        }
        self.num
    }
}

fn index_of(b: u8) -> usize {
    usize::from(b - b'a')
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let wf = WordFilter::new(vec!["apple".into()]);
        debug_assert_eq!(wf.f("a".into(), "e".into()), 0); // return 0
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
