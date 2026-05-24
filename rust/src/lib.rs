mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn password_strength(password: &str) -> i32 {
    use std::collections::HashSet;
    let mut res = 0;
    let mut seen = HashSet::new();
    for b in password.bytes() {
        if seen.insert(b) {
            if b.is_ascii_lowercase() {
                res += 1
            }
            if b.is_ascii_uppercase() {
                res += 2
            }
            if b.is_ascii_digit() {
                res += 3
            }
            if b"!@#$".contains(&b) {
                res += 5
            }
        }
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
        assert_eq!(password_strength("vqztn2Z"), 10);
    }

    #[test]
    fn test() {}
}
