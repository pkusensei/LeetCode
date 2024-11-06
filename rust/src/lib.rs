mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_sort_array(nums: &[i32]) -> bool {
    if nums.is_sorted() {
        true
    } else {
        let mut max = 0;
        for ch in nums.chunk_by(|a, b| a.count_ones() == b.count_ones()) {
            let curr_min = *ch.iter().min().unwrap();
            let curr_max = *ch.iter().max().unwrap();
            if max < curr_min {
                max = curr_max
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_sort_array(&[8, 4, 2, 30, 15]));
        debug_assert!(can_sort_array(&[1, 2, 3, 4, 5]));
        debug_assert!(!can_sort_array(&[3, 16, 8, 4, 2]));
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
