mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_increase(nums: &[i32]) -> i64 {
    let n = nums.len();
    if n & 1 == 1 {
        return f(&nums, 1);
    }
    let mut memo = vec![[-1; 2]; n];
    dfs(&nums, 1, 0, &mut memo).min(dfs(&nums, 2, 1, &mut memo))
}

fn dfs(nums: &[i32], idx: usize, skip: usize, memo: &mut Vec<[i64; 2]>) -> i64 {
    let n = nums.len();
    if idx >= n - 1 {
        return 0;
    }
    if memo[idx][skip] > -1 {
        return memo[idx][skip];
    }
    let curr = i64::from((1 + nums[idx - 1].max(nums[1 + idx])).max(nums[idx]) - nums[idx]);
    if skip > 0 {
        let res = curr + dfs(nums, 2 + idx, 1, memo);
        memo[idx][skip] = res;
        return res;
    }
    let skip_v = curr + dfs(nums, 3 + idx, 1, memo);
    let take = curr + dfs(nums, 2 + idx, 0, memo);
    memo[idx][1] = skip_v;
    memo[idx][0] = take;
    skip_v.min(take)
}

// (max number of indices, min ops)
fn f(nums: &[i32], start: usize) -> i64 {
    let mut ops = 0;
    let n = nums.len();
    for i in (start..n - 1).step_by(2) {
        ops += i64::from((1 + nums[i - 1].max(nums[1 + i])).max(nums[i]) - nums[i]);
    }
    ops
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
        assert_eq!(min_increase(&[2, 1, 1, 3]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(min_increase(&[12, 23, 13, 17, 21, 3]), 0);
    }
}
