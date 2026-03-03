mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_bit(n: i32, k: i32) -> char {
    (f(n, k) + b'0').into()
}

const fn f(n: i32, k: i32) -> u8 {
    if n == 1 {
        return 0;
    }
    let len = (1 << n) - 1;
    let half = (1 + len) / 2;
    if k < half {
        f(n - 1, k)
    } else if k == half {
        1
    } else {
        1 - f(n - 1, len - k + 1)
    }
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
