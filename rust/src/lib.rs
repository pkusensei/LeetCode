mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(words: &[&str]) -> i64 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for s in words {
        let s = s.as_bytes();
        let diff = s
            .iter()
            .map(|&b| (i32::from(b) - i32::from(s[0])).rem_euclid(26))
            .collect_vec();
        *freq.entry(diff).or_insert(0) += 1;
    }
    freq.into_values().map(|v| v * (v - 1) / 2).sum()
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
        assert_eq!(count_pairs(&["fusion", "layout"]), 1);
        assert_eq!(count_pairs(&["ab", "aa", "za", "aa"]), 2);
    }

    #[test]
    fn test() {}
}
