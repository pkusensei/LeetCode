mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut map = HashMap::new();
    let max_len = nums.iter().map(|v| 1 + v.ilog10()).max().unwrap_or(0) as usize;
    for &num in nums.iter() {
        for val in one_swap(num, max_len) {
            res += map.get(&val).unwrap_or(&0)
        }
        *map.entry(num).or_insert(0) += 1;
    }
    res
}

fn one_swap(mut num: i32, n: usize) -> HashSet<i32> {
    let mut res = HashSet::from([num]);
    let mut ds = vec![];
    while num > 0 {
        ds.push(num % 10);
        num /= 10;
    }
    while ds.len() < n {
        ds.push(0);
    }
    ds.reverse();
    for a in 0..n {
        for b in 1 + a..n {
            ds.swap(a, b);
            res.insert(ds.iter().fold(0, |acc, v| acc * 10 + v));
            ds.swap(a, b);
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
