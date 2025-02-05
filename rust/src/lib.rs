mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(nums: &[i32], multipliers: &[i32]) -> i32 {
    let n = nums.len();
    let m = multipliers.len();
    let mut dp = vec![vec![0; 1 + m]; 1 + m];
    for idx in (0..m).rev() {
        for left in (0..=idx).rev() {
            let right = n - 1 - (idx - left);
            let a = multipliers[idx] * nums[left] + dp[1 + idx][1 + left];
            let b = multipliers[idx] * nums[right] + dp[1 + idx][left];
            dp[idx][left] = a.max(b);
        }
    }
    dp[0][0]
    // dfs(nums, multipliers, 0, 0, &mut vec![vec![i32::MIN; m]; m])
}

fn dfs(nums: &[i32], multipliers: &[i32], idx: usize, left: usize, memo: &mut [Vec<i32>]) -> i32 {
    if idx >= multipliers.len() {
        return 0;
    }
    // the moves of left and right sum up to m
    // i.e right moves in range [n-1-m..=n-1] => end - (moves-left)
    let right = nums.len() - 1 - (idx - left);
    if left == right {
        return multipliers[idx] * nums[left];
    }
    if memo[idx][left] > i32::MIN {
        return memo[idx][left];
    }
    let mut res = multipliers[idx] * nums[left] + dfs(nums, multipliers, 1 + idx, 1 + left, memo);
    res = res.max(multipliers[idx] * nums[right] + dfs(nums, multipliers, 1 + idx, left, memo));
    memo[idx][left] = res;
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(maximum_score(&[1, 2, 3], &[3, 2, 1]), 14);
        assert_eq!(
            maximum_score(&[-5, -3, -3, -2, 7, 1], &[-10, -5, 3, 4, 6]),
            102
        );
    }

    #[test]
    fn test() {}
}
