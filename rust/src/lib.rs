mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Default)]
struct SORTracker {
    left: BinaryHeap<Loc>,
    right: BinaryHeap<Reverse<Loc>>,
    idx: usize,
}

impl SORTracker {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, name: String, score: i32) {
        self.left.push(Loc { score, name });
        if self.left.len() > 1 + self.idx {
            let v = self.left.pop().unwrap();
            self.right.push(Reverse(v));
        }
    }

    fn get(&mut self) -> String {
        let res = self.left.peek().unwrap().name.to_string();
        self.idx += 1;
        if let Some(Reverse(v)) = self.right.pop() {
            self.left.push(v);
        }
        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Loc {
    score: i32,
    name: String,
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Loc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then(self.name.cmp(&other.name))
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
