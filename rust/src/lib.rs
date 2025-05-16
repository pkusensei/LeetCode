mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(nums: Vec<i32>) -> i32 {
    let n = nums.iter().map(|v| 1 + v.ilog10()).max().unwrap_or(1);
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    let mut res = 0;
    for &num in &nums {
        two_swaps(num, n as _, &mut set);
        for val in &set {
            res += map.get(val).unwrap_or(&0);
        }
        *map.entry(num).or_insert(0) += 1;
    }
    res
}

fn two_swaps(num: i32, n: usize, set: &mut HashSet<i32>) {
    set.clear();
    set.insert(num);
    one_swap(num, n, set);
    for val in set.clone() {
        one_swap(val, n, set);
    }
}

fn one_swap(mut num: i32, n: usize, set: &mut HashSet<i32>) {
    let mut ds = Vec::with_capacity(n);
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
            set.insert(ds.iter().fold(0, |acc, v| acc * 10 + v));
            ds.swap(a, b);
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
