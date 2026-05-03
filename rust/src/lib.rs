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

pub fn sum_of_primes_in_range(n: i32) -> i32 {
    let rev: i32 = n
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();
    let a = P.partition_point(|&v| v < rev.min(n));
    let b = P.partition_point(|&v| v <= rev.max(n));
    P[a..b].iter().sum()
}

const MAX: usize = 1_000;
static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut sieve = vec![true; 1 + MAX];
    sieve[..2].fill(false);
    for p in 2..=MAX {
        if sieve[p] {
            for val in (p * p..=MAX).step_by(p) {
                sieve[val] = false
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v { Some(i as i32) } else { None })
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
    fn basics() {
        assert_eq!(sum_of_primes_in_range(10), 17);
        assert_eq!(sum_of_primes_in_range(8), 0);
    }

    #[test]
    fn test() {}
}
