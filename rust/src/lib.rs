mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn reorganize_string(s: &str) -> String {
    let mut heap: BinaryHeap<_> = s
        .bytes()
        .fold(HashMap::new(), |mut acc, b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .map(|(b, count)| (count, b))
        .collect();
    let mut res = vec![];
    while let Some((count, b)) = heap.pop() {
        if res.last().is_some_and(|&v| v == b) {
            let Some((c, b2)) = heap.pop() else {
                return String::new(); // not able to deplete all bytes
            };
            res.push(b2);
            if c > 1 {
                heap.push((c - 1, b2));
            }
            heap.push((count, b));
        } else {
            res.push(b);
            if count > 1 {
                heap.push((count - 1, b));
            }
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reorganize_string("aab"), "aba");
        debug_assert_eq!(reorganize_string("aaab"), "");
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
