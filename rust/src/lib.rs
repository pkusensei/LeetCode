mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|val| {
            if let Err(i) = P.binary_search(val) {
                let mut curr = 5000;
                if let Some(v) = P.get(i) {
                    curr = curr.min(v - val);
                }
                if let Some(v) = i.checked_sub(1).map(|i| P[i]) {
                    curr = curr.min(val - v);
                }
                curr
            } else {
                0
            }
        })
        .collect()
}

static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut res = vec![];
    for num in 1..=10_000 {
        if is_palindrome(format!("{:b}", num).bytes()) {
            res.push(num);
        }
    }
    res
});

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
