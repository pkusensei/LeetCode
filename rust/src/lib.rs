mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_positive(arr: &[i32], k: i32) -> i32 {
    let n = arr.len();
    let [mut left, mut right] = [0, n];
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] - mid as i32 - 1 < k {
            left = mid + 1;
        } else {
            right = mid
        }
    }
    k + left as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(find_kth_positive(&[2, 3, 4, 7, 11], 5), 9);
        assert_eq!(find_kth_positive(&[1, 2, 3, 4], 2), 6);
    }

    #[test]
    fn test() {
        assert_eq!(find_kth_positive(&[3, 10], 2), 2);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
