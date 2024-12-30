mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn print_vertically(s: &str) -> Vec<String> {
    let words: Vec<_> = s.split_whitespace().map(|v| v.as_bytes()).collect();
    let len = words.iter().map(|w| w.len()).max().unwrap_or(1);
    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        let mut curr: Vec<_> = words.iter().map(|w| *w.get(i).unwrap_or(&b' ')).collect();
        while curr.last().is_some_and(|&v| v.is_ascii_whitespace()) {
            curr.pop();
        }
        res.push(String::from_utf8(curr).unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(print_vertically("HOW ARE YOU"), ["HAY", "ORO", "WEU"]);
        assert_eq!(
            print_vertically("TO BE OR NOT TO BE"),
            ["TBONTB", "OEROOE", "   T"]
        );
        assert_eq!(
            print_vertically("CONTEST IS COMING"),
            ["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]
        );
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
