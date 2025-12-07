mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_prime(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }
    let mut sieve = vec![true; 1 + n as usize]; // assume all are prime
    sieve[..2].fill(false); // 0 and 1 are not prime
    sieve[2] = true;
    for p in 2..=n as usize {
        if sieve[p] {
            // For this prime, all of its multiples are non-prime
            for val in (p * p..=n as usize).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    let mut prefix = 0;
    let mut res = 0;
    for (p, &is_prime) in sieve.iter().enumerate() {
        if is_prime {
            prefix += p;
        }
        if prefix > n as usize {
            break;
        }
        if sieve[prefix] {
            res = res.max(prefix)
        }
    }
    res as i32
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
