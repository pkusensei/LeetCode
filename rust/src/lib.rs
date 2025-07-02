mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn partition_string(s: String) -> Vec<String> {
    use std::collections::HashSet;
    let n = s.len();
    let mut res = vec![];
    let mut seen = HashSet::new();
    let mut left = 0;
    let mut right = 1;
    while right <= n {
        while right <= n && !seen.insert(&s[left..right]) {
            right += 1;
        }
        if right > n {
            break;
        }
        res.push(&s[left..right]);
        left = right;
        right += 1;
    }
    res.into_iter().map(|s| s.to_owned()).collect()
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
