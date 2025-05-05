mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn clear_stars(s: String) -> String {
    use itertools::Itertools;
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut heap = BinaryHeap::new();
    for (i, b) in s.bytes().enumerate() {
        if b == b'*' {
            heap.pop();
        } else {
            heap.push((Reverse(b), i));
        }
    }
    heap.into_iter()
        .sorted_unstable_by_key(|v| v.1)
        .map(|v| char::from(v.0.0))
        .collect()
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
