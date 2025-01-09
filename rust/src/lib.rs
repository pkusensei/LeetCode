mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_times_all_blue(flips: &[i32]) -> i32 {
    let mut right = 0;
    let mut res = 0;
    for (idx, &num) in flips.iter().enumerate() {
        right = right.max(num);
        if right == 1 + idx as i32 {
            res += 1;
        }
        // flips is a permutation of [1..=n]
        // right is the rightmost set bit
        // right == 1+idx means all numbers in [1..right] are set
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_times_all_blue(&[3, 2, 4, 1, 5]), 2);
        assert_eq!(num_times_all_blue(&[4, 1, 2, 3]), 1);
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
