mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn similar_pairs(words: Vec<String>) -> i32 {
    use itertools::Itertools;
    words
        .iter()
        .map(|s| s.bytes().fold(0, |acc, b| acc | (1 << (b - b'a'))))
        .counts()
        .into_values()
        .map(|v| (v * (v - 1) / 2) as i32)
        .sum()
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
