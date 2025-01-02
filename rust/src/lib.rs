mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn vowel_strings(words: &[&str], queries: &[[i32; 2]]) -> Vec<i32> {
    const VOWELS: &[u8] = b"aeiou";
    let mut prefix = Vec::with_capacity(words.len());
    for word in words.iter() {
        let s = word.as_bytes();
        prefix.push(
            i32::from(VOWELS.contains(&s[0]) && VOWELS.contains(s.last().unwrap()))
                + prefix.last().unwrap_or(&0),
        );
    }
    let mut res = Vec::with_capacity(queries.len());
    for q in queries {
        let (left, right) = (q[0] as usize, q[1] as usize);
        if left == 0 {
            res.push(prefix[right]);
        } else {
            res.push(prefix[right] - prefix[left - 1]);
        }
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
            vowel_strings(&["aba", "bcb", "ece", "aa", "e"], &[[0, 2], [1, 4], [1, 1]]),
            [2, 3, 0]
        );
        assert_eq!(
            vowel_strings(&["a", "e", "i"], &[[0, 2], [0, 1], [2, 2]]),
            [3, 2, 1]
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
