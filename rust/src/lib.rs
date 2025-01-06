mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let h = f64::from(minutes) / 2.0 + f64::from(hour * 30);
    let m = f64::from(minutes * 6);
    let diff = (h - m).abs();
    if diff <= 180.0 {
        diff
    } else {
        360.0 - diff
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(angle_clock(12, 30), 165.0);
        assert_eq!(angle_clock(3, 30), 75.0);
        assert_eq!(angle_clock(3, 15), 7.5);
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
