mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn word_subsets(words1: &[&str], words2: &[&str]) -> Vec<String> {
    let freq = words2
        .iter()
        .map(|s| count(s))
        .fold([0; 26], |mut acc, curr| {
            for (v, c) in acc.iter_mut().zip(curr) {
                (*v) = (*v).max(c);
            }
            acc
        });
    words1
        .iter()
        .filter_map(|s| {
            let curr = count(s);
            if curr.into_iter().zip(freq).all(|(c, f)| c >= f) {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn count(s: &str) -> [i16; 26] {
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
        assert_eq!(
            word_subsets(
                &["amazon", "apple", "facebook", "google", "leetcode"],
                &["e", "o"]
            ),
            ["facebook", "google", "leetcode"]
        );
        assert_eq!(
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
