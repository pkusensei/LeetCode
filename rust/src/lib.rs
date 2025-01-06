mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_steps(s: &str, t: &str) -> i32 {
    let [a, b] = [s, t].map(|v| {
        v.bytes().fold([0i32; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
    });
    (a.into_iter()
        .zip(b)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
        / 2) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_steps("bab", "aba"), 1);
        assert_eq!(min_steps("leetcode", "practice"), 5);
        assert_eq!(min_steps("anagram", "mangaar"), 0);
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
