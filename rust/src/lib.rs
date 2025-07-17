mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::BTreeMap;
struct LRUCache {
    id_keys: BTreeMap<i32, usize>,
    data: Vec<(i32, i32)>, // key - (id, val)
    cap: usize,
    len: usize,
    id: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        Self {
            id_keys: BTreeMap::new(),
            data: vec![(-1, -1); 10_001],
            cap,
            len: 0,
            id: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let key = key as usize;
        if self.data[key].0 < 0 {
            return -1;
        }
        let (id, res) = self.data[key];
        self.data[key].0 = self.id;
        self.id_keys.remove(&id);
        self.id_keys.insert(self.id, key);
        self.id += 1;
        res
    }

    fn put(&mut self, key: i32, value: i32) {
        let key = key as usize;
        let (id, _) = self.data[key];
        self.data[key] = (self.id, value);
        self.id_keys.remove(&id);
        self.id_keys.insert(self.id, key);
        self.id += 1;
        self.len += usize::from(id < 0); // neg => empty => new item
        if self.len > self.cap {
            let del = self.id_keys.pop_first().unwrap().1;
            self.data[del] = (-1, -1);
            self.len -= 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
