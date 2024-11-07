mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips_mono_incr(s: &str) -> i32 {
    let mut zeros: i32 = s.bytes().map(|b| i32::from(b == b'0')).sum();
    let mut ones = 0;
    let mut res = i32::MAX;
    for b in s.bytes() {
        if b == b'0' {
            zeros -= 1;
            res = res.min(zeros + ones);
        } else {
            res = res.min(zeros + ones);
            ones += 1;
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
        debug_assert_eq!(min_flips_mono_incr("00110"), 1);
        debug_assert_eq!(min_flips_mono_incr("010110"), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(min_flips_mono_incr("0101100011"), 3);
        debug_assert_eq!(min_flips_mono_incr("10011111110010111011"), 5);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
