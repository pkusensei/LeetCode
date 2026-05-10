mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_array_sum(nums: Vec<i32>) -> i64 {
    let set: HashSet<_> = nums.iter().copied().collect();
    let mut res = 0;
    for &num in nums.iter() {
        let mut curr = num;
        for div in 1..=num.isqrt() {
            if num % div == 0 {
                if set.contains(&div) {
                    curr = div;
                    break;
                }
                let v = num / div;
                if set.contains(&v) {
                    curr = curr.min(v);
                }
            }
        }
        res += i64::from(curr);
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
