mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

// j - i != nums[j] - nums[i]
// j - nums[j] != i - nums[i]
pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let mut seen = HashMap::new();
    for (i, &num) in (0..).zip(nums.iter()) {
        *seen.entry(i - num).or_insert(0) += 1;
    }
    let n = nums.len() as i64;
    n * (n - 1) / 2 - seen.into_values().map(|v| v * (v - 1) / 2).sum::<i64>()
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
