mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_good_subarrays(nums: &[i32], k: i32) -> i64 {
    use std::collections::HashMap;
    // start from empty subarr (sum % k == 0)
    let mut freq = HashMap::from([(0, 1)]);
    let mut prefix = 0;
    let mut res = 0;
    for &num in nums.iter() {
        prefix = (prefix + num) % k;
        let f = freq.entry(prefix).or_insert(0);
        res += *f;
        *f += 1;
    }
    let k = i64::from(k);
    for ch in nums.chunk_by(|a, b| a == b) {
        let num = i64::from(ch[0]);
        for len in 1..ch.len() {
            if len as i64 * num % k == 0 {
                // This uniform subarr is over-counted
                // Its total in `res` is (n-len+1)
                // Hence remove that (n-len)
                res -= (ch.len() - len) as i64;
            }
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
    fn basics() {
        assert_eq!(num_good_subarrays(&[1, 2, 3], 3), 3);
        assert_eq!(num_good_subarrays(&[2, 2, 2, 2, 2, 2], 6), 2);
    }

    #[test]
    fn test() {
        assert_eq!(num_good_subarrays(&[1, 1, 1, 1, 1, 3, 4, 4, 6, 8], 6), 7);
    }
}
