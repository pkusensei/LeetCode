mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_change(start: &str, target: &str) -> bool {
    let [start, target] =
        [start, target].map(|v| v.bytes().enumerate().filter(|&(_i, b)| b != b'_'));
    if start.clone().count() != target.clone().count() {
        return false;
    }
    for (s, t) in start.zip(target) {
        match (s.1, t.1) {
            (b'L', b'L') => {
                if s.0 < t.0 {
                    return false;
                }
            }
            (b'R', b'R') => {
                if s.0 > t.0 {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(can_change("_L__R__R_", "L______RR"));
        assert!(!can_change("R_L_", "__LR"));
        assert!(!can_change("_R", "R_"));
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
