mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_odd_length_subarrays(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut prefix = Vec::with_capacity(n);
    for num in arr.iter() {
        prefix.push(num + prefix.last().unwrap_or(&0));
    }
    let mut res = 0;
    for len in (1..=n).step_by(2) {
        let mut left = 0;
        let mut win: i32 = prefix[left + len - 1];
        res += win;
        left += 1;
        while left + len <= n {
            win -= arr[left - 1];
            win += arr[left + len - 1];
            res += win;
            left += 1;
        }
    }
    res
}

pub fn with_math(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut res = 0;
    for (left, num) in arr.iter().enumerate() {
        let right = n - left - 1;
        let odd_left = left as i32 / 2 + 1;
        let odd_right = right as i32 / 2 + 1;
        let even_left = (left as i32 + 1) / 2;
        let even_right = (right as i32 + 1) / 2;
        res += num * (odd_left * odd_right + even_left * even_right);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(sum_odd_length_subarrays(&[1, 4, 2, 5, 3]), 58);
        assert_eq!(sum_odd_length_subarrays(&[1, 2]), 3);
        assert_eq!(sum_odd_length_subarrays(&[10, 11, 12]), 66);

        assert_eq!(with_math(&[1, 4, 2, 5, 3]), 58);
        assert_eq!(with_math(&[1, 2]), 3);
        assert_eq!(with_math(&[10, 11, 12]), 66);
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
