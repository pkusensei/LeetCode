mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    dfs(&nums, k as usize, 0, n, i32::MAX, &mut HashMap::new())
}

fn dfs(
    nums: &[i32],
    k: usize,
    idx: usize,
    prev: usize,
    min_d: i32,
    memo: &mut HashMap<(usize, usize, usize, i32), i32>,
) -> i32 {
    let n = nums.len();
    if k == 0 {
        return min_d;
    }
    if n - idx < k {
        return 0;
    }
    let key = (k, idx, prev, min_d);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let curr = if prev == n {
        min_d
    } else {
        min_d.min(nums[idx] - nums[prev])
    };
    let skip = dfs(nums, k, 1 + idx, prev, min_d, memo);
    let take = dfs(nums, k - 1, 1 + idx, idx, curr, memo);
    let res = (skip + take) % 1_000_000_007;
    memo.insert(key, res);
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
        assert_eq!(sum_of_powers(vec![1, 2, 3, 4], 3), 4);
        assert_eq!(sum_of_powers(vec![2, 2], 2), 0);
        assert_eq!(sum_of_powers(vec![4, 3, -1], 2), 10);
    }

    #[test]
    fn test() {}
}
