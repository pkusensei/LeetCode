mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_subarray_mins(arr: &[i32]) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = arr.len();
    let mut dp = vec![0; n];
    let mut stack = vec![];
    for (idx, &num) in arr.iter().enumerate() {
        while stack.last().is_some_and(|&i| arr[i] >= num) {
            stack.pop();
        }
        if let Some(&i) = stack.last() {
            // ..i..idx where [i] < [idx]
            // take current num for [..idx]
            // add in all that account for [..i]
            // Consider [3,1,5,2]
            // dp[1] is the result for all with [3,1]
            // When current num == 2, dp[3] is
            // dp[1] this is all subarrays that starts with [3,1] or [1]
            // (3-1) * 2 is result for [5,2]
            dp[idx] = (dp[i] + (idx - i) as i32 * num) % MOD;
        } else {
            // current num is the min
            // each past subarray should be counted in
            dp[idx] = (1 + idx) as i32 * num % MOD;
        }
        stack.push(idx);
    }
    dp.into_iter().fold(0, |acc, v| (acc + v) % MOD)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(sum_subarray_mins(&[3, 1, 2, 4]), 17);
        debug_assert_eq!(sum_subarray_mins(&[11, 81, 94, 43, 3]), 444);
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
