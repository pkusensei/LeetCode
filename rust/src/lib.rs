mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn repeated_n_times(nums: &[i32]) -> i32 {
    let mut s = std::collections::HashSet::new();
    for &num in nums.iter() {
        if !s.insert(num) {
            return num;
        }
    }
    -1
}

fn pigeon_hole(nums: &[i32]) -> i32 {
    for k in 1..4 {
        for i in 0..nums.len() - k {
            if nums[i] == nums[i + k] {
                return nums[i];
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(pigeon_hole(&[1, 2, 3, 3]), 3);
        debug_assert_eq!(pigeon_hole(&[2, 1, 2, 5, 3, 2]), 2);
        debug_assert_eq!(pigeon_hole(&[5, 1, 5, 2, 5, 3, 5, 4]), 5);
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
