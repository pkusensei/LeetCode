mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_strongest(arr: &mut [i32], mut k: i32) -> Vec<i32> {
    arr.sort_unstable();
    let n = arr.len();
    let median = arr[(n - 1) / 2];
    let [mut left, mut right] = [0, n - 1];
    let mut res = Vec::with_capacity(k as usize);
    while k > 0 {
        if arr[right].abs_diff(median) >= arr[left].abs_diff(median) {
            res.push(arr[right]);
            right -= 1;
        } else {
            res.push(arr[left]);
            left += 1;
        }
        k -= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(get_strongest(&mut [1, 2, 3, 4, 5], 2), [5, 1]);
        assert_eq!(get_strongest(&mut [1, 1, 3, 5, 5], 2), [5, 5]);
        assert_eq!(get_strongest(&mut [6, 7, 11, 7, 6, 8], 5), [11, 8, 6, 6, 7]);
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
