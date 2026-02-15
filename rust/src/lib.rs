mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn add_binary(a: String, b: String) -> String {
    let mut res = vec![];
    let mut carry = 0;
    for it in a.bytes().rev().zip_longest(b.bytes().rev()) {
        let curr = match it {
            itertools::EitherOrBoth::Both(a, b) => a - b'0' + b - b'0',
            itertools::EitherOrBoth::Left(v) | itertools::EitherOrBoth::Right(v) => v - b'0',
        };
        carry += curr;
        res.push((carry & 1) + b'0');
        carry >>= 1;
    }
    if carry > 0 {
        res.push(carry + b'0');
    }
    res.reverse();
    String::from_utf8(res).unwrap()
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
