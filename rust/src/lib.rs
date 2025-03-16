mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct NumberContainers {
    id_num: HashMap<i32, i32>,
    num_ids: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        Default::default()
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&v) = self.id_num.get(&index) {
            if let Some(set) = self.num_ids.get_mut(&v) {
                set.remove(&index);
                if set.is_empty() {
                    self.num_ids.remove(&v);
                }
            }
        }
        self.id_num.insert(index, number);
        self.num_ids.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        self.num_ids
            .get(&number)
            .and_then(|set| set.first().copied())
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
