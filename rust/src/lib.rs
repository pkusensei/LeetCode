mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut zeros = nums.iter().filter(|&&v| v == 0).count();
    let mut res = 0;
    while zeros < n {
        for v in nums.iter_mut() {
            if *v & 1 == 1 {
                *v ^= 1;
                res += 1;
                zeros += usize::from(*v == 0);
            }
        }
        if zeros == n {
            break;
        }
        for v in nums.iter_mut() {
            *v >>= 1;
        }
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_operations(&mut [1, 5]), 5);
        assert_eq!(min_operations(&mut [2, 2]), 3);
        assert_eq!(min_operations(&mut [4, 2, 5]), 6);
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
