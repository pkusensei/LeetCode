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
    if n <= 1 {
        return 0;
    }
    let mut prefix = 0;
    let mut res = 0;
    for p in S.iter() {
        prefix += p;
        if prefix > n {
            break;
        }
        if S.binary_search(&prefix).is_ok() {
            res = res.max(prefix)
        }
    }
    res
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
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v { Some(i as i32) } else { None })
        .collect()
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
