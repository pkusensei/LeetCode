mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn partition_labels(s: &str) -> Vec<i32> {
    let (s, n) = (s.as_bytes(), s.len());
    let lasts = s
        .iter()
        .enumerate()
        .rev()
        .fold(HashMap::new(), |mut acc, (i, &b)| {
            acc.entry(b).or_insert(i);
            acc
        });
    let mut res = vec![];
    let mut left = 0;
    while left < n {
        let mut last = lasts[&s[left]];
        for (right, b) in s.iter().enumerate().skip(left) {
            if right >= last {
                res.push((right - left + 1) as i32);
                left = 1 + right;
                break;
            } else {
                let temp = lasts[b];
                if temp > last {
                    last = temp
                }
            }
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
        debug_assert_eq!(partition_labels("ababcbacadefegdehijhklij"), [9, 7, 8]);
        debug_assert_eq!(partition_labels("eccbbbbdec"), [10]);
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
