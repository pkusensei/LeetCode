mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

#[allow(unused_imports)]
use helper::*;
use itertools::{Itertools, izip};

pub fn max_total(value: &[i32], limit: &[i32]) -> i64 {
    let n = value.len();
    // (value, limit)
    let vals = izip!(value.iter(), limit.iter())
        .map(|(&v, &lim)| (v, lim))
        .sorted_unstable_by_key(|&(v, lim)| (lim, Reverse(v)))
        .collect_vec();
    let mut res = 0;
    let mut active = 0;
    let mut heap = BinaryHeap::new();
    let mut idx = 0;
    while idx < n {
        let (val, lim) = vals[idx];
        if active < lim {
            res += i64::from(val);
            active += 1;
            heap.push(Reverse(lim));
            let i = vals.partition_point(|&(_, _lim)| _lim <= active);
            let temp = active;
            while heap.peek().is_some_and(|&Reverse(v)| v <= temp) {
                heap.pop();
                active -= 1;
            }
            idx = (idx + 1).max(i);
        }
    }
    res
}

pub fn with_deque(value: &[i32], limit: &[i32]) -> i64 {
    // (value, limit)
    let vals = izip!(value.iter(), limit.iter())
        .map(|(&v, &lim)| (v, lim))
        .sorted_unstable_by_key(|&(v, lim)| (lim, Reverse(v)))
        .collect_vec();
    let mut active = VecDeque::new();
    let mut inactive = 0;
    let mut res = 0;
    for (val, lim) in vals {
        let lim = lim as usize;
        if active.len() >= lim || inactive >= lim {
            continue;
        }
        res += i64::from(val);
        active.push_back(lim);
        if active.len() > inactive {
            inactive = active.len();
            while let Some(&f) = active.front()
                && f <= inactive
            {
                active.pop_front();
            }
        }
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
    fn basics() {
        assert_eq!(max_total(&[3, 5, 8], &[2, 1, 3]), 16);
        assert_eq!(max_total(&[4, 2, 6], &[1, 1, 1]), 6);
        assert_eq!(max_total(&[4, 1, 5, 2], &[3, 3, 2, 3]), 12);

        assert_eq!(with_deque(&[3, 5, 8], &[2, 1, 3]), 16);
        assert_eq!(with_deque(&[4, 2, 6], &[1, 1, 1]), 6);
        assert_eq!(with_deque(&[4, 1, 5, 2], &[3, 3, 2, 3]), 12);
    }

    #[test]
    fn test() {}
}
