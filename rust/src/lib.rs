mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BinaryHeap, HashMap};

#[derive(Default)]
struct FreqStack {
    id: u32,
    map: HashMap<i32, i32>,            // val - freq
    heap: BinaryHeap<(i32, u32, i32)>, // (freq, id, val)
}

impl FreqStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        let id = self.id;
        self.id += 1;
        let f = self.map.entry(val).or_default();
        *f += 1;
        self.heap.push((*f, id, val));
    }

    fn pop(&mut self) -> i32 {
        while let Some(top) = self.heap.peek()
            && self.map.get(&top.2).is_none_or(|v| top.0 != *v)
        {
            self.heap.pop();
        }
        let top = self.heap.pop().unwrap_or_default();
        let v = self.map.get_mut(&top.2).unwrap();
        *v -= 1;
        if *v == 0 {
            self.map.remove(&top.2);
        }
        top.2
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
