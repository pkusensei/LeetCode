mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_ways(s: &str) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let sum: i32 = s.bytes().map(|b| i32::from(b == b'1')).sum();
    if sum % 3 > 0 {
        return 0;
    }
    if sum == 0 {
        let n = s.len() as i64;
        return ((n - 1) * (n - 2) / 2 % MOD) as _; // (n-1) choose 2
    }
    let mut prefix = 0;
    let [mut left, mut right] = [0, 0];
    for b in s.bytes() {
        prefix += i32::from(b == b'1');
        if sum / 3 == prefix {
            left += 1;
        }
        if 2 * sum / 3 == prefix {
            right += 1;
        }
    }
    (left * right % MOD) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_ways("10101"), 4);
        assert_eq!(num_ways("1001"), 0);
        assert_eq!(num_ways("0000"), 3);
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
