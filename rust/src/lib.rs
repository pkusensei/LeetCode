mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[allow(unused_imports)]
use helper::*;

struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    out: HashSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            heap: (1..=1000).map(Reverse).collect(),
            out: HashSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        let val = self.heap.pop().unwrap().0;
        self.out.insert(val);
        val
    }

    fn add_back(&mut self, num: i32) {
        if self.out.remove(&num) {
            self.heap.push(Reverse(num));
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
