mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_num_of_valid_words(words: &[&str], puzzles: &[&str]) -> Vec<i32> {
    let words = words
        .iter()
        .map(|s| to_bits(s))
        .fold(HashMap::new(), |mut acc, mask| {
            *acc.entry(mask).or_insert(0) += 1;
            acc
        });
    puzzles
        .iter()
        .map(|p| {
            let i = p.as_bytes()[0] - b'a';
            let mask = to_bits(p);
            let mut res = 0;
            for (&k, &v) in words.iter() {
                if (k >> i) & 1 == 1 && (mask & k) == k {
                    res += v;
                }
            }
            res
        })
        .collect()
}

fn to_bits(s: &str) -> i32 {
    s.bytes().fold(0, |acc, b| acc | 1 << (b - b'a'))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_num_of_valid_words(
                &["aaaa", "asas", "able", "ability", "actt", "actor", "access"],
                &["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"],
            ),
            [1, 1, 3, 2, 4, 0]
        );
        assert_eq!(
            find_num_of_valid_words(
                &["apple", "pleas", "please"],
                &["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"]
            ),
            [0, 1, 3, 2, 0]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
