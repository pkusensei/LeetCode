mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_product(words: &[&str]) -> i32 {
    let bits: Vec<_> = words
        .iter()
        .enumerate()
        .map(|(i, s)| (i, as_bits(s)))
        .collect();
    let mut res = 0;
    for &(i, b1) in bits.iter() {
        for &(j, b2) in bits.iter().skip(i) {
            if b1 & b2 == 0 {
                res = res.max(words[i].len() * words[j].len())
            }
        }
    }
    res as _
}

fn as_bits(s: &str) -> i32 {
    s.bytes().fold(0, |acc, b| acc | 1 << (b - b'a'))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_product(&["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]),
            16
        );
        debug_assert_eq!(
            max_product(&["a", "ab", "abc", "d", "cd", "bcd", "abcd"]),
            4
        );
        debug_assert_eq!(max_product(&["a", "aa", "aaa", "aaaa"]), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
