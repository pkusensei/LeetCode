mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_good_array(nums: &[i32]) -> bool {
    if nums.len() == 1 {
        nums[0] == 1
    } else {
        nums[1..].iter().fold(nums[0], |acc, &v| gcd(acc, v)) == 1
    }
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(is_good_array(&[12, 5, 7, 23]));
        assert!(is_good_array(&[29, 6, 10]));
        assert!(!is_good_array(&[3, 6]));
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
