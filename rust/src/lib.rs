mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MapSum {
    trie: Trie<26>,
}

impl MapSum {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.trie.insert(key.bytes(), Self::index_of(), val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.trie.search(prefix.bytes(), Self::index_of())
    }

    fn index_of() -> impl FnMut(u8) -> usize + Copy {
        |b| usize::from(b - b'a')
    }
}

#[derive(Debug, Clone)]
pub struct Trie<const N: usize> {
    data: [Option<Box<Trie<N>>>; N],
    val: i32,
}

impl<const N: usize> Trie<N> {
    pub const fn new() -> Self {
        Self {
            data: [const { None }; N],
            val: 0,
        }
    }

    pub fn insert<T, I, F>(&mut self, input: I, index_of: F, val: i32)
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        self.insert_impl(input.into_iter(), index_of, val)
    }

    fn insert_impl<T, I, F>(&mut self, mut it: I, mut index_of: F, val: i32)
    where
        I: Iterator<Item = T>,
        F: FnMut(T) -> usize,
    {
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(n) = self.data.get_mut(idx).and_then(|opt| opt.as_mut()) {
                n.insert(it, index_of, val);
            } else {
                let mut node = Box::new(Self::new());
                node.insert(it, index_of, val);
                self.data[idx] = Some(node);
            }
        } else {
            self.val = val;
        }
    }

    pub fn search<T, I, F>(&self, mut it: I, mut index_of: F) -> i32
    where
        I: Iterator<Item = T> + Clone,
        F: FnMut(T) -> usize + Copy,
    {
        if let Some(v) = it.next() {
            let idx = index_of(v);
            if let Some(node) = self.data.get(idx).and_then(|n| n.as_ref()) {
                return node.search(it, index_of);
            }
            return 0;
        }
        let mut res = self.val;
        for n in self.data.iter().filter_map(|opt| opt.as_ref()) {
            res += n.search(std::iter::empty(), index_of);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut ms = MapSum::new();
        ms.insert("apple".into(), 3);
        debug_assert_eq!(ms.sum("ap".into()), 3); // return 3 (apple = 3)
        debug_assert_eq!(ms.sum("apples".into()), 0);
        ms.insert("app".into(), 2);
        debug_assert_eq!(ms.sum("ap".into()), 5); // return 5 (apple + app = 3 + 2 = 5)
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
