mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut i1 = arr1.len();
    let mut i2 = arr2.len();
    let mut carry = 0;
    while i1 > 0 || i2 > 0 {
        if i1 > 0 {
            carry += arr1[i1 - 1];
            i1 -= 1;
        }
        if i2 > 0 {
            carry += arr2[i2 - 1];
            i2 -= 1;
        }
        res.push(carry & 1);
        carry = -(carry >> 1);
    }
    while carry != 0 {
        res.push(carry & 1);
        carry = -(carry >> 1);
    }
    while res.last().is_some_and(|&v| v == 0) {
        res.pop();
    }
    res.reverse();
    if res.is_empty() { vec![0] } else { res }
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
