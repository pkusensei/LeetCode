mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn large_group_positions(s: &str) -> Vec<[i32; 2]> {
    let mut res = vec![];
    let mut idx = 0;
    for ch in s.as_bytes().chunk_by(|a, b| *a == *b) {
        if ch.len() >= 3 {
            res.push([idx as i32, (idx + ch.len() - 1) as i32]);
        }
        idx += ch.len();
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(large_group_positions("abbxxxxzzy"), [[3, 6]]);
        debug_assert!(large_group_positions("abc").is_empty());
        debug_assert_eq!(
            large_group_positions("abcdddeeeeaabbbcd"),
            [[3, 5], [6, 9], [12, 14]]
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
