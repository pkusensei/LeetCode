mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_product(nums: &[i32], k: i32, limit: i32) -> i32 {
    dfs(nums, k, limit, 1, 0, true, true, &mut HashMap::new())
}

fn dfs(
    nums: &[i32],
    k: i32,
    limit: i32,
    prod: i32,
    idx: usize,
    is_even: bool,
    is_empty: bool,
    memo: &mut HashMap<(i32, i32, usize, bool), i32>,
) -> i32 {
    if idx >= nums.len() {
        if !is_empty && k == 0 && prod <= limit {
            return prod;
        }
        return -1;
    }
    let key = (k, prod, idx, is_even);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let skip = dfs(nums, k, limit, prod, 1 + idx, is_even, is_empty, memo);
    let sign = if is_even { 1 } else { -1 };
    let take = dfs(
        nums,
        k - sign * nums[idx],
        limit,
        (prod * nums[idx]).min(1 + limit), // pruning
        1 + idx,
        !is_even,
        false,
        memo,
    );
    let res = skip.max(take);
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
        assert_eq!(max_product(&[1, 2, 3], 2, 10), 6);
        assert_eq!(max_product(&[0, 2, 3], -5, 12), -1);
        assert_eq!(max_product(&[2, 2, 3, 3], 0, 9), 9);
    }

    #[test]
    fn test() {
        assert_eq!(max_product(&[1, 4, 7, 10], -3, 20), 4);
        assert_eq!(max_product(&[6, 3, 3], 6, 20), 6);
    }
}
