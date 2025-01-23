mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reorder_spaces(text: &str) -> String {
    let count = text.bytes().filter(|&b| b == b' ').count();
    let words: Vec<_> = text.split_whitespace().collect();
    let n = words.len();
    let mut res = String::with_capacity(text.len());
    if n <= 1 {
        res.push_str(words[0]);
        res.extend(std::iter::repeat(' ').take(count));
    } else {
        res = words.join(&" ".repeat(count / (n - 1)));
        res.extend(std::iter::repeat(' ').take(count % (n - 1)));
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            reorder_spaces("  this   is  a sentence "),
            "this   is   a   sentence"
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
