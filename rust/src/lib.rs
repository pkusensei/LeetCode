mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn fair_candy_swap(alice_sizes: &[i32], bob_sizes: &[i32]) -> Vec<i32> {
    let sa: i32 = alice_sizes.iter().sum();
    let sb: i32 = bob_sizes.iter().sum();
    let delta = sa - sb;
    let b: std::collections::HashSet<_> = bob_sizes.iter().copied().collect();
    for &a in alice_sizes.iter() {
        if b.contains(&(a - delta / 2)) {
            return vec![a, a - delta / 2];
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(fair_candy_swap(&[1, 1], &[2, 2]), [1, 2]);
        debug_assert_eq!(fair_candy_swap(&[1, 2], &[2, 3]), [1, 2]);
        debug_assert_eq!(fair_candy_swap(&[2], &[1, 3]), [2, 3]);
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
