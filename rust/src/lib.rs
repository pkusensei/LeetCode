mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Default)]
struct NumberContainers {
    num_indices: HashMap<i32, BTreeSet<i32>>,
    index_num: HashMap<i32, i32>,
}

impl NumberContainers {
    fn new() -> Self {
        Default::default()
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&prev) = self.index_num.get(&index) {
            self.num_indices.entry(prev).or_default().remove(&index);
        }
        self.index_num.insert(index, number);
        self.num_indices.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        self.num_indices
            .get(&number)
            .and_then(|set| set.first())
            .copied()
            .unwrap_or(-1)
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
