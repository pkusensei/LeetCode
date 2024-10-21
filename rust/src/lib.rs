mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn letter_case_permutation(s: &str) -> Vec<String> {
    let n = s.len();
    let total = 1 << n;
    let mut res = HashSet::new();
    for mask in 0..total {
        let mut curr = vec![];
        for (i, b) in s.bytes().enumerate() {
            if (mask >> i) & 1 == 1 {
                curr.push(b.to_ascii_uppercase());
            } else {
                curr.push(b.to_ascii_lowercase());
            }
        }
        res.insert(String::from_utf8(curr).unwrap());
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            letter_case_permutation("a1b2"),
            ["a1b2", "a1B2", "A1b2", "A1B2"],
        );
        sort_eq(letter_case_permutation("3z4"), ["3z4", "3Z4"]);
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
