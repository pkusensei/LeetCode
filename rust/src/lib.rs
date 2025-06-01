mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

// p*r == q*s
// p/q == s/r
pub fn number_of_subsequences(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut res = 0;
    let mut map: HashMap<[i32; 2], i64> = HashMap::new();
    for r in 4..n - 2 {
        let q = r - 2;
        for p in 0..q - 1 {
            let [a, b] = [p, q].map(|i| nums[i]);
            let gcd_ = gcd(a, b);
            *map.entry([a / gcd_, b / gcd_]).or_insert(0) += 1;
        }
        for s in 2 + r..n {
            let [a, b] = [s, r].map(|i| nums[i]);
            let gcd_ = gcd(a, b);
            res += map.get(&[a / gcd_, b / gcd_]).unwrap_or(&0);
        }
    }
    res as i64
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(number_of_subsequences(&[1, 2, 3, 4, 3, 6, 1]), 1);
    }

    #[test]
    fn test() {}
}
