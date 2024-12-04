mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_substring(s: String) -> String {
    // let (n, mut s) = (s.len(), s.into_bytes());
    // let (mut i1, mut i2) = (0, 1);
    // let mut len = 0;
    // while i2 + len < n {
    //     if s[i1 + len] == s[i2 + len] {
    //         len += 1;
    //         continue;
    //     }
    //     if s[i1 + len] > s[i2 + len] {
    //         i2 += 1;
    //     } else {
    //         i1 = i2;
    //         i2 = 1 + i1;
    //     }
    //     len = 0;
    // }
    // unsafe { String::from_utf8_unchecked(s.split_off(i1)) }

    // smh everything TLEs except this
    let mut result = &s[..];
    for i in 1..s.len() {
        let candidate = &s[i..];
        if candidate > result {
            result = candidate
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(last_substring("abab".into()), "bab");
        assert_eq!(last_substring("leetcode".into()), "tcode");
    }

    #[test]
    fn test() {
        assert_eq!(last_substring("cacacb".into()), "cb");
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
