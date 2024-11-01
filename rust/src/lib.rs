mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn len_longest_fib_subseq(arr: &[i32]) -> i32 {
    let n = arr.len();
    let map = arr
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, &num)| {
            acc.insert(num, i);
            acc
        });
    let mut dp = vec![vec![1; n]; n];
    for i1 in 0..n - 2 {
        for i2 in 1 + i1..n - 1 {
            if let Some(&i3) = map.get(&(arr[i1] + arr[i2])) {
                dp[i2][i3] = dp[i2][i3].max(1 + dp[i1][i2]).max(3);
            }
        }
    }
    dp.into_iter()
        .flatten()
        .max()
        .filter(|&v| v >= 3)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(len_longest_fib_subseq(&[1, 2, 3, 4, 5, 6, 7, 8]), 5);
        debug_assert_eq!(len_longest_fib_subseq(&[1, 3, 7, 11, 12, 14, 18]), 3);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            len_longest_fib_subseq(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 12, 17, 19, 27, 29]),
            5
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
