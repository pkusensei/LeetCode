mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn score_of_parentheses(s: String) -> i32 {
    f(s.as_bytes())
}

fn f(s: &[u8]) -> i32 {
    if s == b"()" {
        return 1;
    }
    let n = s.len();
    let mut open = 0;
    for (i, &b) in s.iter().enumerate() {
        open += if b == b'(' { 1 } else { -1 };
        if open == 0 {
            if i == n - 1 {
                return 2 * f(&s[1..n - 1]);
            } else {
                return f(&s[..=i]) + f(&s[1 + i..]);
            }
        }
    }
    0
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
    fn basics() {}

    #[test]
    fn test() {}
}
