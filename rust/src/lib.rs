mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_squareful_perms(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    if n == 1 {
        return 1;
    }
    nums.sort_unstable();
    let mut dp = vec![vec![-1; n]; 1 << n];
    let mut res = 0;
    for i in 0..n {
        res += backtrack(nums, 1 << i, i, &mut dp);
    }
    res
}

fn backtrack(nums: &[i32], mask: usize, prev: usize, dp: &mut [Vec<i32>]) -> i32 {
    let n = nums.len();
    if mask == (1 << n) - 1 {
        return 1;
    }
    if dp[mask][prev] > -1 {
        return dp[mask][prev];
    }
    let mut res = 0;
    for idx in 0..n {
        if (mask >> idx) & 1 == 1 {
            continue;
        }
        if idx > 0 && nums[idx] == nums[idx - 1] && (mask >> (idx - 1)) & 1 == 1 {
            continue;
        }
        if is_square(nums[prev] + nums[idx]) {
            res += backtrack(nums, mask | (1 << idx), idx, dp);
        }
    }
    dp[mask][prev] = res;
    res
}

fn is_square(num: i32) -> bool {
    let r = f64::from(num).sqrt().floor() as i32;
    r * r == num
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_squareful_perms(&mut [1, 17, 8]), 2);
        debug_assert_eq!(num_squareful_perms(&mut [2, 2, 2]), 1);
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
