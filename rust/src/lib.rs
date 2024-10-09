mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MagicDictionary {
    trie: Trie<26>,
}

impl MagicDictionary {
    const fn new() -> Self {
        Self { trie: Trie::new() }
    }

    fn build_dict(&mut self, dictionary: &[&str]) {
        for s in dictionary.iter() {
            self.trie.insert(s.bytes(), |b| usize::from(b - b'a'));
        }
    }

    fn search(&self, s: &str) -> bool {
        self.trie.search(s.bytes(), |b| usize::from(b - b'a'), 0)
    }
}

#[derive(Debug, Clone)]
pub struct Trie<const N: usize> {
    data: [Option<Box<Trie<N>>>; N],
    is_end: bool,
}

impl<const N: usize> Trie<N> {
    pub const fn new() -> Self {
        Self {
            data: [const { None }; N],
            is_end: false,
        }
    }

    pub fn insert<T, I, F>(&mut self, input: I, index_of: F)
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.insert_impl(input.into_iter(), index_of)
    }

    fn insert_impl<T, I, F>(&mut self, mut it: I, mut index_of: F)
    where
        I: Iterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it, index_of);
            } else {
                let mut node = Box::new(Self::new());
                node.insert(it, index_of);
                self.data[idx] = Some(node);
            }
        } else {
            self.is_end = true;
        }
    }

    pub fn search<T, I, F>(&self, mut it: I, mut index_of: F, missed: i32) -> bool
    where
        I: Iterator<Item = T> + Clone,
        F: FnMut(T) -> usize + Copy,
    {
        if missed > 1 {
            return false;
        }
        if let Some(v) = it.next() {
            let idx = index_of(v);
            let mut res = false;
            for (i, node) in self
                .data
                .iter()
                .enumerate()
                .filter_map(|(i, opt)| opt.as_ref().map(|v| (i, v)))
            {
                if i == idx {
                    res |= node.search(it.clone(), index_of, missed);
                } else {
                    res |= node.search(it.clone(), index_of, 1 + missed)
                }
            }
            return res;
        }
        missed == 1 && self.is_end
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut md = MagicDictionary::new();
        md.build_dict(&["hello", "leetcode"]);
        debug_assert!(!md.search("hello")); // return False
        debug_assert!(md.search("hhllo")); // We can change the second 'h' to 'e' to match "hello" so we return True
        debug_assert!(!md.search("hell")); // return False
        debug_assert!(!md.search("leetcoded")); // return False

        md.trie.insert("hallo".bytes(), |b| usize::from(b - b'a'));
        debug_assert!(md.search("hello"));
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
