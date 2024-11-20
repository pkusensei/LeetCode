mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k & 1 == 0 || k % 5 == 0 {
        return -1;
    }
    let mut n = 0;
    // let mut set = std::collections::HashSet::new();
    // there can be at most (k-1) rems
    for len in 1..=k {
        n = 10 * n + 1;
        n %= k;
        if n == 0 {
            return len;
        }
        // if !set.insert(n) {
        //     break;
        // }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_repunit_div_by_k(1), 1);
        debug_assert_eq!(smallest_repunit_div_by_k(2), -1);
        debug_assert_eq!(smallest_repunit_div_by_k(3), 3);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
