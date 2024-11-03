mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn orderly_queue(s: String, k: i32) -> String {
    if k >= 2 {
        let mut s = s.into_bytes();
        s.sort_unstable();
        String::from_utf8(s).unwrap()
    } else {
        let n = s.len();
        let v = format!("{s}{s}").into_bytes();
        v.windows(n)
            .min()
            .and_then(|s| String::from_utf8(s.to_vec()).ok())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(orderly_queue("cba".into(), 1), "acb");
        debug_assert_eq!(orderly_queue("baaca".into(), 3), "aaabc");
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
