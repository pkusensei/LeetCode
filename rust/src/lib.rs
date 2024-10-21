mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn num_matching_subseq(s: &str, words: &[&str]) -> i32 {
    let mut map = HashMap::new();
    // with_trie(words, s)
    let mut res = 0;
    for &word in words.iter() {
        if map.get(word).is_some_and(|&v| v) {
            res += 1;
        } else {
            let t = is_subseq(s.as_bytes(), word.as_bytes());
            map.insert(word, t);
            res += i32::from(t);
        }
    }
    res
}

fn is_subseq(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    let mut idx = 0;
    for &b in hay.iter() {
        if needle[idx] == b {
            idx += 1;
            if idx == needle.len() {
                return true;
            }
        }
    }
    false
}

fn with_trie(words: &[&str], s: &str) -> i32 {
    let words = words.iter().fold(HashMap::new(), |mut acc, w| {
        *acc.entry(*w).or_insert(0) += 1;
        acc
    });
    let mut trie = Trie::default();
    trie.insert(s.bytes());
    words
        .into_iter()
        .filter_map(|(k, v)| if trie.check(k.bytes()) { Some(v) } else { None })
        .sum()
}

#[derive(Debug, Clone, Default)]
struct Trie {
    data: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub const fn new() -> Self {
        Self {
            data: [const { None }; 26],
        }
    }

    pub fn insert<I>(&mut self, input: I)
    where
        I: IntoIterator<Item = u8>,
    {
        self.insert_impl(input.into_iter())
    }

    fn insert_impl<I>(&mut self, mut it: I)
    where
        I: Iterator<Item = u8>,
    {
        if let Some(v) = it.next() {
            let idx = Self::index_of(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it);
            } else {
                let mut node = Box::new(Self::new());
                node.insert(it);
                self.data[idx] = Some(node);
            }
        }
    }

    fn check<I>(&self, mut it: I) -> bool
    where
        I: Iterator<Item = u8> + Clone,
    {
        let temp = it.clone();
        if let Some(v) = it.next() {
            let idx = Self::index_of(v);
            if let Some(n) = self.data.get(idx).and_then(|opt| opt.as_ref()) {
                return n.check(it);
            } else {
                return self
                    .data
                    .iter()
                    .filter_map(|n| n.as_ref())
                    .next()
                    .is_some_and(|n| n.check(temp));
            }
        }
        true
    }

    fn index_of(b: u8) -> usize {
        usize::from(b - b'a')
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_matching_subseq("abcde", &["a", "bb", "acd", "ace"]), 3);
        debug_assert_eq!(
            num_matching_subseq("dsahjpjauf", &["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]),
            2
        );
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
