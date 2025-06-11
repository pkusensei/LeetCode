mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum(mut grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    for (row, limit) in grid.iter_mut().zip(limits) {
        if limit > 0 {
            row.select_nth_unstable_by_key(limit as usize - 1, |&v| Reverse(v));
            heap.extend(&row[..limit as usize]);
        }
    }
    let mut res = 0;
    for _ in 0..k {
        let Some(v) = heap.pop().map(i64::from) else {
            break;
        };
        res += v;
    }
    res
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
