mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_integer(num: String, k: i32) -> String {
    let mut s = num.into_bytes();
    let n = s.len();
    let mut k = k as usize;
    for left in 0..n - 1 {
        if k == 0 {
            break;
        }
        let right = s
            .iter()
            .enumerate()
            .skip(left)
            .take(1 + k)
            .min_by_key(|(_i, v)| **v)
            .map(|(i, _v)| i)
            .unwrap_or(left);
        if right == left {
            continue;
        }
        s[left..=right].rotate_right(1);
        k -= right - left;
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_integer("4321".into(), 4), "1342");
        assert_eq!(min_integer("100".into(), 1), "010");
        assert_eq!(min_integer("36789".into(), 1000), "36789");
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
