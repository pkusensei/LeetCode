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

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        if idx & 1 == 0 {
            let i = P.partition_point(|&v| v < num);
            res += P[i] - num;
        } else {
            if num == 2 {
                res += 2
            } else if P.binary_search(&num).is_ok() {
                res += 1
            }
        }
    }
    res
}

const MAX: usize = 100_003;

static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut sieve = vec![true; 1 + MAX];
    sieve[..2].fill(false);
    for p in 2..=MAX {
        if sieve[p] {
            for val in (p * p..=MAX).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i as i32) } else { None })
        .collect()
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
