mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn closest_to_target(arr: &[i32], target: i32) -> i32 {
    let mut set = HashSet::new();
    let mut res = i32::MAX;
    for &num in arr.iter() {
        // Generate bitand values on the fly
        // In each loop, the new num is plugged in
        // Plus its bitand-ed with all previous bitand results
        let mut next = HashSet::from([num]);
        next.extend(set.into_iter().map(|v| v & num));
        set = next;
        for v in set.iter() {
            res = res.min((v - target).abs());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(closest_to_target(&[9, 12, 3, 7, 15], 5), 2);
        assert_eq!(closest_to_target(&[1000000, 1000000, 1000000], 1), 999999);
        assert_eq!(closest_to_target(&[1, 2, 4, 8, 16], 0), 0);
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
