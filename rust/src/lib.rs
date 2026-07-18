mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_value(n: i32, s: i32, m: i32) -> i64 {
    let [n, s, m] = [n, s, m].map(i64::from);
    if n == 1 {
        return s;
    }
    let count = (1 + n) / 2;
    let last = s + count * (m - 1);
    if n & 1 == 0 { last + 1 } else { last + 2 - m }
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
        assert_eq!(maximum_value(3, 3, 5), 8);
        assert_eq!(maximum_value(4, 3, 5), 12);
        assert_eq!(maximum_value(2, 4, 3), 7);
    }

    #[test]
    fn test() {}
}
