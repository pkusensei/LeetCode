mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
    let n = nums.len();
    let [op1, op2] = [op1, op2].map(|v| v as usize);
    dfs(
        &nums,
        k,
        0,
        op1,
        op2,
        &mut vec![vec![vec![-1; 1 + op2]; 1 + op1]; n],
    )
}

fn dfs(
    nums: &[i32],
    k: i32,
    idx: usize,
    op1: usize,
    op2: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    if idx >= nums.len() {
        return 0;
    }
    if memo[idx][op1][op2] > -1 {
        return memo[idx][op1][op2];
    }
    let val = nums[idx];
    let mut res = val + dfs(nums, k, 1 + idx, op1, op2, memo); // skip
    if op1 > 0 {
        let curr = (1 + val) / 2;
        res = res.min(curr + dfs(nums, k, 1 + idx, op1 - 1, op2, memo));
        if curr >= k && op2 > 0 {
            res = res.min(curr - k + dfs(nums, k, 1 + idx, op1 - 1, op2 - 1, memo));
        }
    }
    if op2 > 0 && val >= k {
        let curr = val - k;
        res = res.min(curr + dfs(nums, k, 1 + idx, op1, op2 - 1, memo));
        if op1 > 0 {
            res = res.min((1 + curr) / 2 + dfs(nums, k, 1 + idx, op1 - 1, op2 - 1, memo));
        }
    }
    memo[idx][op1][op2] = res;
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
    fn test() {}
}
