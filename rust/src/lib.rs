mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let sieve = sieve(right);
    let mut diff = right;
    let mut res = vec![-1, -1];
    for w in sieve.windows(2) {
        let d = w[1] - w[0];
        if left <= w[0] && d < diff {
            diff = d;
            res = w.to_vec();
            if diff == 2 {
                break;
            }
        }
    }
    res
}

fn sieve(n: i32) -> Vec<i32> {
    let mut primes = vec![true; 1 + n as usize];
    primes[0..2].copy_from_slice(&[false; 2]);
    for p in 2..=(n as f64).sqrt() as usize {
        if primes[p] {
            for i in (p * p..=n as usize).step_by(p) {
                primes[i] = false;
            }
        }
    }
    primes
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
    fn basics() {
        assert_eq!(closest_primes(10, 19), [11, 13]);
        assert_eq!(closest_primes(4, 6), [-1, -1]);
    }

    #[test]
    fn test() {
        assert_eq!(closest_primes(19, 31), [29, 31]);
    }
}
