mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn word_subsets<'a>(words1: &[&'a str], words2: &[&'a str]) -> Vec<&'a str> {
    let counts = words2
        .iter()
        .map(|s| count(s))
        .fold([0; 26], |mut acc, curr| {
            for (a, c) in acc.iter_mut().zip(curr) {
                *a = (*a).max(c);
            }
            acc
        });
    words1
        .iter()
        .filter(|s| {
            count(s)
                .into_iter()
                .zip(counts.iter())
                .all(|(a, &b)| a >= b)
        })
        .copied()
        .collect()
}

fn count(s: &str) -> [i32; 26] {
    s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            word_subsets(
                &["amazon", "apple", "facebook", "google", "leetcode"],
                &["e", "o"]
            ),
            ["facebook", "google", "leetcode"]
        );
        debug_assert_eq!(
            word_subsets(
                &["amazon", "apple", "facebook", "google", "leetcode"],
                &["l", "e"]
            ),
            ["apple", "google", "leetcode"]
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
