mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_jumps(arr: &[i32], d: i32) -> i32 {
    let mut nums: Vec<_> = arr.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    nums.sort_unstable_by_key(|(_i, v)| std::cmp::Reverse(*v));
    let (n, d) = (arr.len(), d as usize);
    let mut dp = vec![1; n];
    for (i1, v) in nums {
        for i2 in (i1.saturating_sub(d)..i1)
            .rev()
            .take_while(|i2| arr[*i2] < v)
        {
            dp[i2] = dp[i2].max(1 + dp[i1]);
        }
        for i2 in (1 + i1..=(i1 + d).min(n - 1)).take_while(|i2| arr[*i2] < v) {
            dp[i2] = dp[i2].max(1 + dp[i1]);
        }
    }
    dp.into_iter().max().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_jumps(&[6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
        assert_eq!(max_jumps(&[3, 3, 3, 3, 3], 3), 1);
        assert_eq!(max_jumps(&[7, 6, 5, 4, 3, 2, 1], 1), 7);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
