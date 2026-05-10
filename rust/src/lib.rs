mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_word_occurrences(chunks: &[&str], queries: &[&str]) -> Vec<i32> {
    let s = chunks.join("").into_bytes();
    let n = s.len();
    let mut words = HashMap::new();
    let mut left = 0;
    for (right, &b) in s.iter().enumerate() {
        if b.is_ascii_lowercase() {
            continue;
        }
        if b == b'-'
            && right > 0
            && s[right - 1].is_ascii_lowercase()
            && right < n - 1
            && s[1 + right].is_ascii_lowercase()
        {
            continue;
        }
        if left < right {
            *words.entry(&s[left..right]).or_insert(0) += 1;
        }
        left = 1 + right;
    }
    if left < n {
        *words.entry(&s[left..]).or_insert(0) += 1;
    }
    queries
        .iter()
        .map(|q| *words.get(q.as_bytes()).unwrap_or(&0))
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
    fn basics() {
        assert_eq!(
            count_word_occurrences(&["a--b a-", "-c"], &["a", "b", "c"]),
            [2, 1, 1]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            count_word_occurrences(&["m  cq-i "], &["m", "cq-i", "nm"]),
            [1, 1, 0]
        )
    }
}
