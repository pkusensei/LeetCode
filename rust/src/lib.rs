mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(nums: Vec<i32>, forbidden: Vec<i32>) -> i32 {
    use itertools::izip;
    use std::collections::HashMap;

    let n = nums.len();
    let mut total_freq = HashMap::new();
    let mut pairs = HashMap::new();
    for (&a, &b) in izip!(nums.iter(), forbidden.iter()) {
        *total_freq.entry(a).or_insert(0) += 1;
        *total_freq.entry(b).or_insert(0) += 1;
        if a == b {
            *pairs.entry(a).or_insert(0) += 1;
        }
    }
    if total_freq.values().any(|&v| v > n) {
        return -1;
    }
    let sum: i32 = pairs.values().sum();
    let max = *pairs.values().max().unwrap_or(&0);
    max.max((sum + 1) / 2)
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
