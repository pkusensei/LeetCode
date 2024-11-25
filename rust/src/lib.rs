mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_after_partitioning(arr: &[i32], k: i32) -> i32 {
    let (n, k) = (arr.len(), k as usize);
    let mut dp = vec![0; 1 + n];
    for len in 1..=n {
        let mut curr_max = 0;
        for i in (1..=k).filter(|i| *i <= len) {
            curr_max = curr_max.max(arr[len.saturating_sub(i)]);
            dp[len] = dp[len].max(dp[len.saturating_sub(i)] + curr_max * i as i32);
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_sum_after_partitioning(&[1, 15, 7, 9, 2, 5, 10], 3), 84);
        debug_assert_eq!(
            max_sum_after_partitioning(&[1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
        debug_assert_eq!(max_sum_after_partitioning(&[1], 1), 1);
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
