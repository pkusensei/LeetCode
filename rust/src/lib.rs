mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut hash = vec![0; 1 + n];
    let mut pows = vec![1; 1 + n];
    for (i, &num) in nums.iter().enumerate() {
        hash[1 + i] = (hash[i] * B % M + i64::from(num)) % M;
        pows[1 + i] = pows[i] * B % M;
    }
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = left + (right - left) / 2;
        if check(&hash, &pows, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left as i32
}

const M: i64 = 1_000_000_007;
const B: i64 = 215215;

fn check(hash: &[i64], pows: &[i64], mid: usize) -> bool {
    use std::collections::HashMap;

    let n = hash.len() - 1;
    let mut freq = HashMap::new();
    for left in 0..=(n - mid) {
        let right = left + mid;
        let h = (hash[right] - hash[left] * pows[mid] % M).rem_euclid(M);
        *freq.entry(h).or_insert(0) += 1;
    }
    freq.into_values().any(|f| f == 1)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
