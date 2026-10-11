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

pub fn max_primes(n: i32, s: i32) -> Vec<i32> {
    let mut sum = s;
    let mut res = vec![];
    for &p in P.iter() {
        if sum < p || n < p {
            break;
        }
        res.push(p);
        sum -= p;
    }
    res
}

const M: usize = 1_000_000;
static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut sieve = vec![true; 1 + M];
    sieve[..2].fill(false);
    for p in 2..=M.isqrt() {
        if sieve[p] {
            for val in (p * p..=M).step_by(p) {
                sieve[val] = false
            }
        }
    }
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v { Some(i as i32) } else { None })
        .collect()
});

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
