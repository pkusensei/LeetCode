mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_subarrays(arr: &[i32]) -> i32 {
    let [mut odd, mut even] = [0, 1]; // zero is even
    let mut sum = 0;
    let mut res = 0;
    for num in arr.iter() {
        sum += num;
        if sum & 1 == 1 {
            res += even;
            odd += 1;
        } else {
            res += odd;
            even += 1;
        }
        res %= 1_000_000_007;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_of_subarrays(&[1, 3, 5]), 4);
        assert_eq!(num_of_subarrays(&[2, 4, 6]), 0);
        assert_eq!(num_of_subarrays(&[1, 2, 3, 4, 5, 6, 7]), 16);
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
