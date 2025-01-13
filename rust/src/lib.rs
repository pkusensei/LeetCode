mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_ways(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut p123 = 6;
    let mut p121 = 6;
    for _ in 1..n {
        // 123 => 212 232 and 231 312
        let next123 = (2 * p121 + 2 * p123) % MOD;
        // 121 => 212 232 313 and 312 213
        let next121 = (3 * p121 + 2 * p123) % MOD;
        [p123, p121] = [next123, next121]
    }
    ((p123 + p121) % MOD) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_of_ways(1), 12);
        assert_eq!(num_of_ways(5000), 30228214);
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
