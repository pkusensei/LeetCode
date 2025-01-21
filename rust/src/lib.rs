mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn thousand_separator(mut n: i32) -> String {
    let mut res = vec![];
    let mut count = 0;
    while n > 0 {
        res.push((n % 10) as u8 + b'0');
        n /= 10;
        count += 1;
        if count == 3 {
            count = 0;
            res.push(b'.');
        }
    }
    if res.last().is_some_and(|&v| v == b'.') {
        res.pop();
    }
    if res.is_empty() {
        return "0".into();
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
