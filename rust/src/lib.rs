mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut sieve = vec![true; 1 + n];
    sieve[..2].fill(false);
    for p in 2..=n {
        if sieve[p] {
            for i in (2 * p..=n).step_by(p) {
                sieve[i] = false;
            }
        }
    }
    let mut res = vec![];
    for a in 2..=n / 2 {
        if sieve[a] && sieve[n - a] {
            res.push(vec![a as i32, (n - a) as i32]);
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
