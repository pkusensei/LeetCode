mod helper;
mod trie;

use std::{cmp::Reverse, iter};

#[allow(unused_imports)]
use helper::*;

pub fn frequency_sort(s: &str) -> String {
    let mut counts: Vec<_> = s
        .bytes()
        .fold([0; 62], |mut acc, b| {
            let i = match b {
                b'A'..=b'Z' => usize::from(b - b'A'),
                b'a'..=b'z' => usize::from(b - b'a' + 26),
                b'0'..=b'9' => usize::from(b - b'0' + 52),
                _ => unreachable!(),
            };
            acc[i] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .map(|(i, count)| match i {
            0..=25 => (b'A' + i as u8, count),
            26..=51 => (b'a' + i as u8 - 26, count),
            52..=61 => (b'0' + i as u8 - 52, count),
            _ => unreachable!(),
        })
        .collect();
    counts.sort_unstable_by_key(|(_b, c)| Reverse(*c));
    counts
        .into_iter()
        .flat_map(|(b, count)| iter::repeat(char::from(b)).take(count))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(frequency_sort("tree"), "eetr");
        debug_assert_eq!(frequency_sort("cccaaa"), "cccaaa");
        debug_assert_eq!(frequency_sort("Aabb"), "bbAa");
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
}
