mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_awesome(s: &str) -> i32 {
    let mut prefix: u16 = 0;
    let mut seen = [i32::MAX; 1 << 10];
    seen[0] = -1;
    let mut res = 0;
    for (i, b) in s.bytes().map(|b| b - b'0').enumerate() {
        prefix ^= 1 << b;
        res = res.max(i as i32 - seen[usize::from(prefix)]);
        for d in 0..=9 {
            res = res.max(i as i32 - seen[usize::from(prefix ^ (1 << d))]);
        }
        seen[usize::from(prefix)] = seen[usize::from(prefix)].min(i as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(longest_awesome("3242415"), 5);
        assert_eq!(longest_awesome("12345678"), 1);
        assert_eq!(longest_awesome("213123"), 6);
    }

    #[test]
    fn test() {
        assert_eq!(longest_awesome("51224"), 3);
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
