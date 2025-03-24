mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
    let mut res = HashSet::new();
    for &num in nums.iter() {
        res.extend(find(num));
    }
    res.len() as _
}

fn find(mut num: i32) -> HashSet<i32> {
    let mut res = HashSet::new();
    let mut p = 2;
    while num > 1 {
        while num % p == 0 {
            num /= p;
            res.insert(p);
        }
        p += 1;
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
