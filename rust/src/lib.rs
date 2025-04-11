mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_interesting_subarrays(nums: &[i32], modulo: i32, k: i32) -> i64 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let n = nums.len();
    let mut prefix = nums
        .iter()
        .map(|v| i32::from(v % modulo == k))
        .collect_vec();
    for i in 1..n {
        prefix[i] += prefix[i - 1];
    }
    let mut map = HashMap::from([(0, 1)]);
    let mut res = 0_i64;
    for &p in prefix.iter() {
        res += map
            .get(&((p + modulo - k).rem_euclid(modulo)))
            .unwrap_or(&0);
        *map.entry(p % modulo).or_insert(0) += 1;
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
    fn basics() {
        assert_eq!(count_interesting_subarrays(&[3, 2, 4], 2, 1), 3);
        assert_eq!(count_interesting_subarrays(&[3, 1, 9, 6], 3, 0), 2);
    }

    #[test]
    fn test() {
        assert_eq!(count_interesting_subarrays(&[11, 12, 21, 31], 10, 1), 5);
    }
}
