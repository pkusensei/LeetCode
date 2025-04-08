mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::{Itertools, izip};

pub fn minimum_time(nums1: &[i32], nums2: &[i32], x: i32) -> i32 {
    let n = nums1.len();
    let nums = izip!(nums1.iter(), nums2.iter())
        .map(|(&a, &b)| [a, b])
        .sorted_unstable_by_key(|v| v[1])
        .collect_vec();
    // number to be reduced
    let mut dp = vec![vec![0; 1 + n]; 1 + n];
    for cnt in 1..=n {
        for ops in 1..=cnt {
            dp[cnt][ops] = dp[cnt - 1][ops]
                .max(dp[cnt - 1][ops - 1] + nums[cnt - 1][1] * ops as i32 + nums[cnt - 1][0]);
        }
    }
    let sum1: i32 = nums1.iter().sum();
    let sum2: i32 = nums2.iter().sum();
    for t in 0..=n {
        if sum1 + t as i32 * sum2 - dp[n][t] <= x {
            return t as i32;
        }
    }
    -1
}

pub fn pick_unpick(nums1: &[i32], nums2: &[i32], x: i32) -> i32 {
    let n = nums1.len();
    let nums = izip!(nums1.iter(), nums2.iter())
        .map(|(&a, &b)| [a, b])
        .sorted_unstable_by_key(|v| v[1])
        .collect_vec();
    let mut dp = vec![vec![-1; 1 + n]; 1 + n];
    let sum1: i32 = nums1.iter().sum();
    let sum2: i32 = nums2.iter().sum();
    for t in 0..=n {
        if sum1 + t as i32 * sum2 - dfs(&nums, Some(n - 1), t, &mut dp) <= x {
            return t as i32;
        }
    }
    -1
}

fn dfs(nums: &[[i32; 2]], idx: Option<usize>, time: usize, dp: &mut [Vec<i32>]) -> i32 {
    if time == 0 {
        return 0;
    }
    let Some(idx) = idx else {
        return 0;
    };
    if dp[idx][time] > -1 {
        return dp[idx][time];
    }
    let skip = dfs(nums, idx.checked_sub(1), time, dp);
    let take =
        nums[idx][0] + time as i32 * nums[idx][1] + dfs(nums, idx.checked_sub(1), time - 1, dp);
    let res = skip.max(take);
    dp[idx][time] = res;
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
        assert_eq!(minimum_time(&[1, 2, 3], &[1, 2, 3], 4), 3);
        assert_eq!(minimum_time(&[1, 2, 3], &[3, 3, 3], 4), -1);

        assert_eq!(pick_unpick(&[1, 2, 3], &[1, 2, 3], 4), 3);
        assert_eq!(pick_unpick(&[1, 2, 3], &[3, 3, 3], 4), -1);
    }

    #[test]
    fn test() {}
}
