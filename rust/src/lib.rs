mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 3 {
        return 1;
    }
    1 + dfs(
        &nums,
        nums[0] + nums[1],
        2,
        n - 1,
        &mut vec![vec![-1; n]; n],
    )
    .max(dfs(
        &nums,
        nums[n - 2] + nums[n - 1],
        0,
        n - 3,
        &mut vec![vec![-1; n]; n],
    ))
    .max(dfs(
        &nums,
        nums[0] + nums[n - 1],
        1,
        n - 2,
        &mut vec![vec![-1; n]; n],
    ))
}

fn dfs(nums: &[i32], sum: i32, left: usize, right: usize, memo: &mut [Vec<i32>]) -> i32 {
    if left >= right {
        return 0;
    }
    if memo[left][right] > -1 {
        return memo[left][right];
    }
    let mut res = 0;
    if nums[left] + nums[1 + left] == sum {
        res = res.max(1 + dfs(nums, sum, 2 + left, right, memo));
    }
    if nums[right - 1] + nums[right] == sum {
        res = res.max(1 + dfs(nums, sum, left, right.saturating_sub(2), memo));
    }
    if nums[left] + nums[right] == sum {
        res = res.max(1 + dfs(nums, sum, 1 + left, right - 1, memo));
    }
    memo[left][right] = res;
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
        assert_eq!(max_operations(vec![1, 1, 1, 1, 1, 1]), 3);
    }
}
