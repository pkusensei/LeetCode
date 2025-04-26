mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
    use std::collections::HashSet;
    let mut res = vec![];
    for (i1, a) in arr.iter().enumerate() {
        let n = a.len();
        let mut set = HashSet::new();
        for left in 0..n {
            for right in 1 + left..=n {
                set.insert(&a[left..right]);
            }
        }
        let mut seen = HashSet::new();
        for (i2, b) in arr.iter().enumerate() {
            if i2 == i1 {
                continue;
            }
            for s in &set {
                if b.contains(s) {
                    seen.insert(*s);
                }
            }
        }
        res.push(
            set.difference(&seen)
                .min_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)))
                .map(|s| s.to_string())
                .unwrap_or_default(),
        );
    }
    res
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
