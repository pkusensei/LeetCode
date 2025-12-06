mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BTreeMap;
    const M: i32 = 1_000_000_007;

    let n = nums.len();
    let mut map = BTreeMap::new();
    let mut left = 0;
    let mut dp = vec![0; 1 + n];
    dp[0] = 1;
    let mut curr = 0;
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        curr = (curr + dp[right]) % M;
        while let (Some(a), Some(b)) = (map.keys().next(), map.keys().next_back())
            && b - a > k
        {
            let v = map.entry(nums[left]).or_insert(0);
            *v -= 1;
            if *v == 0 {
                map.remove(&nums[left]);
            }
            curr = (curr - dp[left]).rem_euclid(M);
            left += 1;
        }
        dp[1 + right] = curr;
    }
    dp[n]
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
