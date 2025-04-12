mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sum(nums: &[i32]) -> i64 {
    use std::collections::HashMap;
    let n = nums.len();
    let sieve = primes(n);
    let mut map = HashMap::new();
    for (mut i, &val) in (1..).zip(nums.iter()) {
        let mut product = 1;
        for &p in sieve.iter() {
            while i % (p * p) == 0 {
                i /= p * p;
            }
            if i % p == 0 {
                product *= p;
            }
        }
        *map.entry(product).or_insert(0) += i64::from(val);
    }
    map.into_values().max().unwrap()
}

fn primes(n: usize) -> Vec<i32> {
    let mut sieve = vec![true; 1 + n];
    sieve[..2].fill(false);
    for p in 2..=n {
        if sieve[p] {
            for val in (2 * p..=n).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v { Some(i as i32) } else { None })
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
        assert_eq!(maximum_sum(&[8, 7, 3, 5, 7, 2, 4, 9]), 16);
        assert_eq!(maximum_sum(&[8, 10, 3, 8, 1, 13, 7, 9, 4]), 20);
    }

    #[test]
    fn test() {}
}
