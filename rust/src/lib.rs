mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn build_array(target: &[i32], _n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut curr = 1;
    for &num in target.iter() {
        while curr < num {
            res.extend(["Push", "Pop"].map(|s| s.to_string()));
            curr += 1;
        }
        res.push("Push".to_string());
        curr += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(build_array(&[1, 3], 3), ["Push", "Push", "Pop", "Push"]);
        assert_eq!(build_array(&[1, 2, 3], 3), ["Push", "Push", "Push"]);
        assert_eq!(build_array(&[1, 2], 4), ["Push", "Push"]);
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
