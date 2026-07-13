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

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let a = NUMS.partition_point(|&v| v < low);
    let b = NUMS.partition_point(|&v| v <= high);
    NUMS[a..b].to_vec()
}

static NUMS: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut res = vec![];
    'out: for len in 2..=9 {
        for start in 1..9 {
            let mut curr = 0;
            let mut d = start;
            let mut len = len;
            while len > 0 {
                if d > 9 {
                    continue 'out;
                }
                curr = curr * 10 + d;
                len -= 1;
                d += 1;
            }
            res.push(curr);
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
