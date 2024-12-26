mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn replace_elements(arr: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(arr.len());
    let mut curr = -1;
    for &num in arr.iter().rev() {
        res.push(curr);
        curr = curr.max(num);
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            replace_elements(&[17, 18, 5, 4, 6, 1]),
            [18, 6, 6, 6, 1, -1]
        );
        assert_eq!(replace_elements(&[400]), [-1]);
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
