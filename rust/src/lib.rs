mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
    let starts: HashSet<_> = start_words.iter().map(|s| to_mask(s)).collect();
    let mut res = 0;
    for tar in target_words.iter() {
        let mask = to_mask(tar);
        for b in tar.bytes() {
            if starts.contains(&(mask - (1 << (b - b'a')))) {
                res += 1;
                break;
            }
        }
    }
    res
}

fn to_mask(s: &str) -> i32 {
    s.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')))
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
