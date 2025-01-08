mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let mut arr = nums.to_vec();
    arr.sort_unstable();
    let map =
        arr.into_iter()
            .enumerate()
            .fold(std::collections::HashMap::new(), |mut acc, (i, num)| {
                acc.entry(num).or_insert(i as i32);
                acc
            });
    nums.iter().map(|v| map[v]).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            smaller_numbers_than_current(&[8, 1, 2, 2, 3]),
            [4, 0, 1, 1, 3]
        );
        assert_eq!(smaller_numbers_than_current(&[6, 5, 4, 8]), [2, 1, 0, 3]);
        assert_eq!(smaller_numbers_than_current(&[7, 7, 7, 7]), [0, 0, 0, 0]);
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
