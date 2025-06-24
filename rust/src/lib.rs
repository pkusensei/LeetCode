mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_largest_primes(s: &str) -> i64 {
    use std::collections::BTreeSet;
    let n = s.len();
    let mut seen = BTreeSet::new();
    for a in 0..n {
        for b in 1 + a..=n {
            let Ok(v) = s[a..b].parse() else {
                continue;
            };
            if is_prime(v) {
                seen.insert(v);
            }
        }
    }
    let mut res = 0;
    for _ in 0..3 {
        let Some(v) = seen.pop_last() else {
            break;
        };
        res += v;
    }
    res
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    for p in 2..=n.isqrt() {
        if n % p == 0 {
            return false;
        }
    }
    true
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
    fn basics() {
        assert_eq!(sum_of_largest_primes("12234"), 1469);
        assert_eq!(sum_of_largest_primes("111"), 11);
    }

    #[test]
    fn test() {}
}
