mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_sequences(nums: &[i32], k: i64) -> i32 {
    use std::collections::HashMap;
    let mut prev = HashMap::from([([k, 1], 1)]);
    for &num in nums.iter().rev() {
        // unchanged
        let mut curr = prev.clone();
        for (&[up, down], &f) in prev.iter() {
            let a = up * i64::from(num);
            let gcd_ = gcd(a, down);
            *curr.entry([a / gcd_, down / gcd_]).or_insert(0) += f;
            let b = down * i64::from(num);
            let gcd_ = gcd(up, b);
            *curr.entry([up / gcd_, b / gcd_]).or_insert(0) += f;
        }
        prev = curr;
    }
    *prev.get(&[1, 1]).unwrap_or(&0)
}

const fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(count_sequences(&[5, 5], 1,), 3);
    }
}
