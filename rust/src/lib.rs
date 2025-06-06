mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_frequency(nums: &[i32], k: i32) -> i32 {
    use std::collections::HashMap;
    let freq = nums.iter().fold(HashMap::new(), |mut acc, &v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });
    let mut res = 0;
    for &target in freq.keys() {
        let mut curr = 0;
        for &num in nums.iter() {
            if num == k {
                curr -= 1; // lose this native `k`
            }
            if num == target {
                curr += 1; // target -> k
            }
            if curr < 0 {
                curr = 0; // reset subarray
            }
            res = res.max(curr);
        }
    }
    res + freq.get(&k).unwrap_or(&0)
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
        assert_eq!(max_frequency(&[1, 2, 3, 4, 5, 6], 1), 2);
        assert_eq!(max_frequency(&[10, 2, 3, 4, 5, 5, 4, 3, 2, 2], 10), 4);
    }

    #[test]
    fn test() {}
}
