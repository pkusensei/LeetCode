mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_maximum_xor(nums: &[i32], maximum_bit: i32) -> Vec<i32> {
    let max = (1 << maximum_bit) - 1;
    let mut xor = 0;
    let mut res = vec![];
    for &num in nums.iter() {
        xor ^= num;
        res.push(max ^ xor);
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_maximum_xor(&[0, 1, 1, 3], 2), [0, 3, 2, 3]);
        debug_assert_eq!(get_maximum_xor(&[2, 3, 4, 7], 3), [5, 2, 6, 5]);
        debug_assert_eq!(get_maximum_xor(&[0, 1, 2, 2, 5, 7], 3), [4, 3, 6, 4, 6, 7]);
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
