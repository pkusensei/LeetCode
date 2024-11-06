mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_by_key(|n| n & 1);
    let i = nums.partition_point(|v| v & 1 == 0);
    let mut right = if i & 1 == 1 { i + 1 } else { i };
    let mut left = 1;
    let n = nums.len();
    while right < n {
        nums.swap(left, right);
        left += 2;
        right += 2
    }
    nums
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(sort_array_by_parity_ii(vec![4, 2, 5, 7]), [4, 5, 2, 7]);
        debug_assert_eq!(sort_array_by_parity_ii(vec![2, 3]), [2, 3]);
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
