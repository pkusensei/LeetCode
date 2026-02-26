mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;

pub fn num_steps(s: String) -> i32 {
    let mut s = s.into_bytes();
    let mut res = 0;
    while s.len() > 1 {
        while s.last().is_some_and(|&b| b == b'0') {
            s.pop();
            res += 1;
        }
        if s.len() == 1 {
            break;
        }
        while s.last().is_some_and(|&b| b == b'1') {
            s.pop();
            res += 1; // these '1's turn into '0's to be popped
        }
        s.pop(); // this '0' becomes 1
        s.push(b'1');
        res += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
