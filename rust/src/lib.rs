mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn top_k_frequent(words: &[&str], k: i32) -> Vec<String> {
    let mut heap: BinaryHeap<_> = words
        .iter()
        .fold(HashMap::new(), |mut acc, s| {
            *acc.entry(*s).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .map(|(s, count)| (count, Reverse(s)))
        .collect();
    let mut res = Vec::with_capacity(k as usize);
    while res.len() < k as usize {
        let Some((_, Reverse(s))) = heap.pop() else {
            break;
        };
        res.push(s.to_string());
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
            top_k_frequent(&["i", "love", "leetcode", "i", "love", "coding"], 2),
            ["i", "love"]
        );
        debug_assert_eq!(
            top_k_frequent(
                &["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"],
                4
            ),
            ["the", "is", "sunny", "day"]
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
}
