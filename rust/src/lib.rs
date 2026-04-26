mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;

pub fn sort_vowels(s: String) -> String {
    use itertools::Itertools;
    use std::{cmp::Reverse, collections::HashMap};
    let mut freq = HashMap::<_, (_, _)>::new();
    for (i, b) in s.bytes().enumerate() {
        if b"aeiou".contains(&b) {
            let v = freq.entry(b).or_insert((i, 0));
            v.1 += 1;
        }
    }
    let mut arr = freq
        .into_iter()
        .map(|(k, (i, f))| (f, i, k))
        .sorted_unstable_by_key(|&(f, i, _byte)| (Reverse(f), i))
        .flat_map(|(f, _i, byte)| repeat_n(byte, f as usize));
    let mut s = s.into_bytes();
    for b in s.iter_mut() {
        if b"aeiou".contains(b) {
            *b = arr.next().unwrap();
        }
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
