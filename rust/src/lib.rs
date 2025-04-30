mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        return false;
    }
    let [mut v, mut c] = [false; 2];
    for b in word.bytes() {
        if b.is_ascii_digit() {
        } else if b.is_ascii_lowercase() {
            if b"aeiou".contains(&b) {
                v = true;
            } else {
                c = true;
            }
        } else if b.is_ascii_uppercase() {
            if b"AEIOU".contains(&b) {
                v = true;
            } else {
                c = true;
            }
        } else {
            return false;
        }
    }
    v && c
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
