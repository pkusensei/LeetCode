mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
    use itertools::Itertools;
    use std::collections::HashSet;
    responses
        .into_iter()
        .map(|row| row.into_iter().collect::<HashSet<_>>())
        .flatten()
        .counts()
        .into_iter()
        .sorted_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))
        .next()
        .unwrap()
        .0
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
