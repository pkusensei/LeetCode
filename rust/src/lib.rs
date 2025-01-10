mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_distance_value(arr1: &[i32], arr2: &mut [i32], d: i32) -> i32 {
    arr2.sort_unstable();
    let mut res = 0;
    for &num in arr1.iter() {
        let a = arr2.partition_point(|&v| v < num - d);
        let b = arr2.partition_point(|&v| v <= num + d);
        res += i32::from(a == b);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_the_distance_value(&[4, 5, 8], &mut [10, 9, 1, 8], 2),
            2
        );
        assert_eq!(
            find_the_distance_value(&[1, 4, 2, 3], &mut [-4, -3, 6, 10, 20, 30], 3),
            2
        );
        assert_eq!(
            find_the_distance_value(&[2, 1, 100, 3], &mut [-5, -2, 10, -3, 7], 6),
            1
        );
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
