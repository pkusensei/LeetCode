mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words
        .iter()
        .flat_map(|word| {
            word.split(separator).filter_map(|s| {
                if !s.is_empty() {
                    Some(s.to_string())
                } else {
                    None
                }
            })
        })
        .collect()
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
