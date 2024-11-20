mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_arith_seq_length(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dp = std::collections::HashMap::new();
    for i1 in 1..n {
        for i2 in 0..i1 {
            let d = nums[i2] - nums[i1];
            let curr = 1 + dp.get(&(i2, d)).copied().unwrap_or(1);
            dp.insert((i1, d), curr);
        }
    }
    dp.into_values().max().unwrap_or(2)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_arith_seq_length(&[3, 6, 9, 12]), 4);
        debug_assert_eq!(longest_arith_seq_length(&[9, 4, 7, 2, 10]), 3);
        debug_assert_eq!(longest_arith_seq_length(&[20, 1, 15, 3, 10, 5, 8]), 4);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            longest_arith_seq_length(&[75, 12, 29, 77, 29, 84, 63, 44, 79, 59, 10]),
            3
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
