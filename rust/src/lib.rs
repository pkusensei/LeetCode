mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_length_encoding(words: &[&str]) -> i32 {
    let mut trie = Trie::new(|b: u8| usize::from(b - b'a'));
    for s in words.iter() {
        trie.insert(s.bytes().rev());
    }
    trie.probe(0)
}

#[derive(Debug, Clone)]
struct Trie<F> {
    data: [Option<Box<Trie<F>>>; 26],
    index_of: F,
}

impl<F> Trie<F> {
    fn new(index_of: F) -> Self {
        Self {
            data: [const { None }; 26],
            index_of,
        }
    }

    fn insert<I>(&mut self, mut it: I)
    where
        I: Iterator<Item = u8>,
        F: Fn(u8) -> usize + Clone,
    {
        if let Some(byte) = it.next() {
            let idx = (self.index_of)(byte);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it);
            } else {
                let mut node = Box::new(Self::new(self.index_of.clone()));
                node.insert(it);
                self.data[idx] = Some(node);
            }
        }
    }

    fn probe(&self, depth: i32) -> i32 {
        if self.data.iter().all(|n| n.is_none()) {
            return 1 + depth; // add in '#'
        }
        self.data
            .iter()
            .filter_map(|n| n.as_ref())
            .map(|n| n.probe(1 + depth))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(minimum_length_encoding(&["time", "me", "bell"]), 10);
        debug_assert_eq!(minimum_length_encoding(&["t"]), 2);
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
