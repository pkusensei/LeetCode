mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(s: &str) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    if s.is_sorted() {
        return 0;
    }
    if n == 2 {
        return -1;
    }
    let min = *s.iter().min().unwrap();
    let max = *s.iter().max().unwrap();
    if s[0] == min || s[n - 1] == max {
        return 1;
    }
    if s[..n - 1].contains(&min) || s[1..].contains(&max) {
        return 2;
    }
    3
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
    fn test() {
        assert_eq!(min_operations("edc"), 3);
        assert_eq!(min_operations("jgg"), 2);
    }
}
