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

pub fn largest_prime(n: i32) -> i32 {
    let i = S.partition_point(|&v| v <= n);
    i.checked_sub(1).map(|i| S[i]).unwrap_or_default()
}

static S: LazyLock<Vec<i32>> = LazyLock::new(precompute);

fn precompute() -> Vec<i32> {
    const N: usize = 500_000;
    let mut sieve = vec![true; 1 + N]; // assume all are prime
    sieve[..2].fill(false); // 0 and 1 are not prime
    sieve[2] = true;
    for p in 2..=N {
        if sieve[p] {
            // For this prime, all of its multiples are non-prime
            for val in (p * p..=N).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    let mut res = vec![];
    let mut prefix = 0;
    for (i, b) in sieve.iter().enumerate() {
        if *b {
            prefix += i;
        }
        if prefix > N {
            break;
        }
        if sieve[prefix] {
            res.push(prefix as i32);
        }
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
