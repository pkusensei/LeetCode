mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game(nums: &[i32]) -> bool {
    let n = nums.len();
    let sum: i32 = nums.iter().sum();
    dfs(nums, 0, n - 1, &mut vec![vec![-1; n]; n]) > sum / 2
    // All of these to day: return true;
}

fn dfs(nums: &[i32], start: usize, end: usize, dp: &mut [Vec<i32>]) -> i32 {
    if start >= end {
        return 0;
    }
    if dp[start][end] > -1 {
        return dp[start][end];
    }
    let a = dfs(nums, 1 + start, end, dp) + nums[start];
    let b = dfs(nums, start, end - 1, dp) + nums[end];
    dp[start][end] = a.max(b);
    dp[start][end]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(stone_game(&[5, 3, 4, 5]));
        debug_assert!(stone_game(&[3, 7, 2, 3]));
    }

    #[test]
    fn test() {}

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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
