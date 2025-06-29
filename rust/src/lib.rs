mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{BTreeMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

const M: i32 = 1_000_000_007;

pub fn count_partitions(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![0; 1 + n];
    dp[0] = 1;
    let mut window = 0;
    let mut map = BTreeMap::new();
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        window = (window + dp[right]) % M;
        *map.entry(num).or_insert(0) += 1;
        while map
            .keys()
            .next()
            .zip(map.keys().last())
            .is_some_and(|(a, b)| b - a > k)
        {
            window = (window - dp[left]).rem_euclid(M);
            let v = map.entry(nums[left]).or_insert(0);
            *v -= 1;
            if *v == 0 {
                map.remove(&nums[left]);
            }
            left += 1;
        }
        dp[1 + right] = window;
    }
    dp[n]
}

pub fn with_deque(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![0; 1 + n];
    dp[0] = 1;
    let mut acc: i32 = 1;
    let mut left = 0;
    let [mut minq, mut maxq] = [const { VecDeque::new() }; 2];
    for (right, &num) in nums.iter().enumerate() {
        while maxq.back().is_some_and(|(_, v)| *v < num) {
            maxq.pop_back();
        }
        maxq.push_back((right, num));
        while minq.back().is_some_and(|(_, v)| *v > num) {
            minq.pop_back();
        }
        minq.push_back((right, num));
        while maxq
            .front()
            .zip(minq.front())
            .is_some_and(|((_, a), (_, b))| a - b > k)
        {
            acc = (acc - dp[left]).rem_euclid(M);
            left += 1;
            if minq.front().is_some_and(|(i, _)| *i < left) {
                minq.pop_front();
            }
            if maxq.front().is_some_and(|(i, _)| *i < left) {
                maxq.pop_front();
            }
        }
        dp[1 + right] = acc;
        acc = (acc + dp[1 + right]) % M;
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
    fn basics() {
        assert_eq!(count_partitions(&[3, 3, 4], 0), 2);
        assert_eq!(count_partitions(&[9, 4, 1, 3, 7], 4), 6);

        assert_eq!(with_deque(&[3, 3, 4], 0), 2);
        assert_eq!(with_deque(&[9, 4, 1, 3, 7], 4), 6);
    }

    #[test]
    fn test() {}
}
