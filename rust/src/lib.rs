mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_str_chain(words: &mut [&str]) -> i32 {
    let n = words.len();
    let mut dp = std::collections::HashMap::with_capacity(n);
    words.sort_unstable_by_key(|s| s.len());
    let mut res = 1;
    for s in words.iter() {
        let mut curr = 1;
        for skip in 0..s.len() {
            let short = format!("{}{}", &s[..skip], &s[1 + skip..]);
            if let Some(&v) = dp.get(short.as_str()) {
                curr = curr.max(1 + v);
            }
        }
        dp.insert(*s, curr);
        res = res.max(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            longest_str_chain(&mut ["a", "b", "ba", "bca", "bda", "bdca"]),
            4
        );
        debug_assert_eq!(
            longest_str_chain(&mut ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]),
            5
        );
        debug_assert_eq!(longest_str_chain(&mut ["abcd", "dbqca"]), 1);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
