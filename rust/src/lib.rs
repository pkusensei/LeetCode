mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindromic_subsequence(s: &str) -> i32 {
    let n = s.len();
    let mut count = [0u16; 26];
    let mut prefix = Vec::with_capacity(n);
    let mut map = std::collections::HashMap::new();
    for (i, b) in s.bytes().enumerate() {
        count[usize::from(b - b'a')] += 1;
        prefix.push(count);
        map.entry(b).or_insert(i);
    }
    let mut res = 0;
    for (i, b) in s.bytes().enumerate().rev() {
        let Some(&left) = map.get(&b).filter(|&&v| v != i) else {
            continue;
        };
        map.remove(&b);
        let (left, mut right) = (prefix[left], prefix[i - 1]);
        for (a, b) in right.iter_mut().zip(left) {
            *a -= b;
        }
        res += right.into_iter().filter(|&v| v > 0).count();
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(count_palindromic_subsequence("aabca"), 3);
        assert_eq!(count_palindromic_subsequence("abc"), 0);
        assert_eq!(count_palindromic_subsequence("bbcbaba"), 4);
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
