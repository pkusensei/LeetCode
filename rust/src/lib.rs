mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_if_exist(arr: &[i32]) -> bool {
    let mut seen = std::collections::HashSet::new();
    for &num in arr.iter() {
        if seen.contains(&(2 * num)) {
            return true;
        }
        if num & 1 == 0 && seen.contains(&(num / 2)) {
            return true;
        }
        seen.insert(num);
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(check_if_exist(&[10, 2, 5, 3]));
        assert!(!check_if_exist(&[3, 1, 7, 11]));
    }

    #[test]
    fn test() {
        assert!(!check_if_exist(&[4, -7, 11, 4, 18]));
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
