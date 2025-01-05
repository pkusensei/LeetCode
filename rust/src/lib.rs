mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_difficulty(job_difficulty: &[i32], d: i32) -> i32 {
    let (n, d) = (job_difficulty.len(), d as usize);
    // dfs(job_difficulty, 0, d, &mut vec![vec![-1; n]; 1 + d]).unwrap_or(-1)
    if n < d {
        return -1;
    }
    let mut dp = vec![vec![i32::MAX; 1 + n]; 1 + d];
    dp[0][0] = 0;
    for i1 in 1..=d {
        for i2 in i1..=n {
            let mut curr_max = 0;
            for i3 in (i1 - 1..i2).rev() {
                curr_max = curr_max.max(job_difficulty[i3]);
                if dp[i1 - 1][i3] != i32::MAX {
                    dp[i1][i2] = dp[i1][i2].min(dp[i1 - 1][i3] + curr_max);
                }
            }
        }
    }
    dp[d][n]
}

fn dfs(nums: &[i32], idx: usize, d: usize, memo: &mut [Vec<i32>]) -> Option<i32> {
    if nums[idx..].len() < d {
        return None;
    }
    if memo[d][idx] > -1 {
        return Some(memo[d][idx]);
    }
    if 1 == d {
        return Some(*nums[idx..].iter().max().unwrap());
    }
    let mut res = i32::MAX;
    let mut curr_max = 0;
    for (i, &num) in nums.iter().enumerate().skip(idx) {
        let Some(right) = dfs(nums, 1 + i, d - 1, memo) else {
            break;
        };
        curr_max = curr_max.max(num);
        res = res.min(curr_max + right);
    }
    if res == i32::MAX {
        None
    } else {
        memo[d][idx] = res;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_difficulty(&[6, 5, 4, 3, 2, 1], 2), 7);
        assert_eq!(min_difficulty(&[9, 9, 9], 4), -1);
        assert_eq!(min_difficulty(&[1, 1, 1], 3), 3);
    }

    #[test]
    fn test() {
        assert_eq!(
            min_difficulty(&[11, 111, 22, 222, 33, 333, 44, 444], 6),
            843
        );
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
