mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn find_valid_split(nums: &[i32]) -> i32 {
    let n = nums.len();
    let maps = nums.iter().map(|&v| primes(v)).collect_vec();
    let mut right = HashMap::new();
    for m in maps.iter() {
        for (&k, v) in m.iter() {
            *right.entry(k).or_insert(0) += v;
        }
    }
    let mut left = HashMap::new();
    for (idx, m) in maps[..n - 1].iter().enumerate() {
        for (&k, v) in m.iter() {
            *left.entry(k).or_insert(0) += v;
            right.entry(k).and_modify(|c| *c -= v);
            if right[&k] == 0 {
                right.remove(&k);
            }
        }
        if left.keys().all(|a| !right.contains_key(a)) {
            return idx as i32;
        }
    }
    -1
}

fn primes(mut num: i32) -> HashMap<i32, i32> {
    let mut res = HashMap::new();
    let sq = num.isqrt();
    for p in 2..=sq {
        while num % p == 0 {
            num /= p;
            *res.entry(p).or_insert(0) += 1;
        }
    }
    if num > 1 {
        *res.entry(num).or_insert(0) += 1;
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
    fn basics() {
        assert_eq!(find_valid_split(&[4, 7, 8, 15, 3, 5]), 2);
        assert_eq!(find_valid_split(&[4, 7, 15, 8, 3, 5]), -1);
    }

    #[test]
    fn test() {}
}
