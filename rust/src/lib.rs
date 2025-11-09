mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(nums: &[i32], k: i32) -> i64 {
    use itertools::{Itertools, chain};
    let k = k as usize;
    let min_idx = nums
        .iter()
        .enumerate()
        .min_by_key(|(_i, v)| **v)
        .map(|(i, _)| i)
        .unwrap_or(0);
    // min at start
    let a = chain!(&nums[min_idx..], &nums[..min_idx])
        .copied()
        .collect_vec();
    // min at end
    let b = chain!(&nums[1 + min_idx..], &nums[..=min_idx])
        .copied()
        .collect_vec();
    rolling_dp(&a, k).max(rolling_dp(&b, k))
}

// 3573
fn rolling_dp(nums: &[i32], k: usize) -> i64 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }
    let mut cash = vec![0; 1 + k];
    let mut normal = vec![i64::MIN >> 1; 1 + k];
    let mut short = vec![i64::MIN >> 1; 1 + k];
    for i in 1..=k {
        normal[i] = -i64::from(nums[0]);
        short[i] = i64::from(nums[0]);
    }
    for &num in nums {
        let num = i64::from(num);
        let mut ncash = cash.clone();
        let mut nnormal = normal.clone();
        let mut nshort = short.clone();
        for i in 1..=k {
            ncash[i] = cash[i].max(normal[i] + num).max(short[i] - num);
            nnormal[i] = normal[i].max(cash[i - 1] - num);
            nshort[i] = short[i].max(cash[i - 1] + num);
        }
        [cash, normal, short] = [ncash, nnormal, nshort];
    }
    cash[k]
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
        assert_eq!(maximum_score(&[1, 2, 3, 3], 2), 3);
        assert_eq!(maximum_score(&[1, 2, 3, 3], 1), 2);
        assert_eq!(maximum_score(&[1, 2, 3, 3], 4), 3);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_score(&[1, 1, 2, 2, 2, 1], 3), 2);
    }
}
