mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::once;

#[allow(unused_imports)]
use helper::*;
use itertools::{chain, izip};

pub fn min_operations(s: String) -> i32 {
    let it = chain!(once(b'0'), once(b'1')).cycle();
    izip!(s.bytes(), it.clone())
        .filter(|(a, b)| a != b)
        .count()
        .min(izip!(s.bytes(), it.skip(1)).filter(|(a, b)| a != b).count()) as i32
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
