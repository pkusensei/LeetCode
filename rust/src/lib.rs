mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct StreamChecker {
    trie: Trie,
    max_len: usize,
    queue: VecDeque<u8>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut s = Self::default();
        for w in words.into_iter() {
            s.max_len = s.max_len.max(w.len());
            s.trie.insert(w.bytes().rev());
        }
        s
    }

    fn query(&mut self, letter: char) -> bool {
        self.queue.push_front(letter as u8);
        self.queue.truncate(self.max_len);
        self.trie.check(self.queue.iter().copied())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Trie {
    data: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    const fn new() -> Self {
        Self {
            data: [const { None }; 26],
            is_end: false,
        }
    }

    fn insert(&mut self, mut it: impl Iterator<Item = u8>) {
        if let Some(v) = it.next() {
            let idx = index(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it);
            } else {
                let mut node = Box::new(Self::new());
                node.insert(it);
                self.data[idx] = Some(node);
            }
        } else {
            self.is_end = true;
        }
    }

    fn check(&self, mut it: impl Iterator<Item = u8>) -> bool {
        if self.is_end {
            return true;
        }
        let Some(idx) = it.next().map(index) else {
            return false;
        };
        if let Some(node) = self.data.get(idx).and_then(|n| n.as_ref()) {
            node.check(it)
        } else {
            false
        }
    }
}

fn index(byte: u8) -> usize {
    usize::from(byte - b'a')
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut s = StreamChecker::new(vec!["cd".into(), "f".into(), "kl".into()]);
        debug_assert!(!s.query('a')); // return False
        debug_assert!(!s.query('b')); // return False
        debug_assert!(!s.query('c')); // return False
        debug_assert!(s.query('d')); // return True, because 'cd' is in the wordlist
        debug_assert!(!s.query('e')); // return False
        debug_assert!(s.query('f')); // return True, because 'f' is in the wordlist
        debug_assert!(!s.query('g')); // return False
        debug_assert!(!s.query('h')); // return False
        debug_assert!(!s.query('i')); // return False
        debug_assert!(!s.query('j')); // return False
        debug_assert!(!s.query('k')); // return False
        debug_assert!(s.query('l')); // return True, because 'kl' is in the wordlist
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
