mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_insertions(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut open = 0;
    let mut i = 0;
    let mut res = 0;
    while i < n {
        if s[i] == b'(' {
            open += 1;
        } else {
            if open > 0 {
                open -= 1;
                if s.get(1 + i).is_some_and(|&v| v == b')') {
                    i += 2;
                    continue;
                } else {
                    res += 1;
                }
            } else {
                res += 1;
                if s.get(1 + i).is_some_and(|&v| v == b')') {
                    i += 2;
                    continue;
                } else {
                    res += 1;
                }
            }
        }
        i += 1;
    }
    res + 2 * open
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(min_insertions("(()))"), 1);
    }

    #[test]
    fn test() {}
}
