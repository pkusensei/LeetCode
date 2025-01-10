mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn create_target_array(nums: &[i32], index: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(nums.len());
    for (&num, &idx) in nums.iter().zip(index.iter()) {
        if idx as usize >= res.len() {
            res.push(num);
        } else {
            res.insert(idx as _, num);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            create_target_array(&[0, 1, 2, 3, 4], &[0, 1, 2, 2, 1]),
            [0, 4, 1, 3, 2]
        );
        assert_eq!(
            create_target_array(&[1, 2, 3, 4, 0], &[0, 1, 2, 3, 0]),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(create_target_array(&[1], &[0]), [1]);
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
