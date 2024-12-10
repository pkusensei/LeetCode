mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subsequence(arr: &[i32], difference: i32) -> i32 {
    let mut dp = HashMap::with_capacity(arr.len());
    for &num in arr.iter() {
        if let Some(&v) = dp.get(&(num - difference)) {
            dp.insert(num, 1 + v);
        } else {
            dp.insert(num, 1);
        }
    }
    dp.into_values().max().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(longest_subsequence(&[1, 2, 3, 4], 1), 4);
        assert_eq!(longest_subsequence(&[1, 3, 5, 7], 1), 1);
        assert_eq!(longest_subsequence(&[1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
