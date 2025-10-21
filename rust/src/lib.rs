mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n1 = nums1.len();
    let n2 = nums2.len();
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    let mut res = 0;
    for (i1, &v1) in nums1.iter().enumerate() {
        for (i2, &v2) in nums2.iter().enumerate() {
            if v1 == v2 {
                dp[1 + i1][1 + i2] = 1 + dp[i1][i2];
                res = res.max(dp[1 + i1][1 + i2]);
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
    fn basics() {}

    #[test]
    fn test() {}
}
