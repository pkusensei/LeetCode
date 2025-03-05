mod dsu;
mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut k = i64::from(k);
    let set: BTreeSet<_> = nums.into_iter().map(i64::from).collect();
    let mut res = k * (1 + k) / 2;
    for num in set {
        if num <= k {
            res -= num;
            k += 1;
            res += k;
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
    fn test() {
        assert_eq!(
            minimal_k_sum(
                vec![53, 41, 90, 33, 84, 26, 50, 32, 63, 47, 66, 43, 29, 88, 71, 28, 83],
                76
            ),
            3444
        );
    }
}
