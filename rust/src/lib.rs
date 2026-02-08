mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp(nums1: &[i32], nums2: &[i32], k: i32) -> i64 {
    let [n1, n2] = [&nums1, &nums2].map(|v| v.len());
    let k = k as usize;
    let mut dp = vec![vec![vec![i64::MIN >> 2; 1 + k]; 1 + n2]; 1 + n1];
    for i1 in 0..n1 {
        for i2 in 0..n2 {
            dp[i1][i2][0] = 0;
            for kk in 0..k {
                dp[1 + i1][1 + i2][1 + kk] = dp[i1][1 + i2][1 + kk]
                    .max(dp[1 + i1][i2][1 + kk])
                    .max(i64::from(nums1[i1]) * i64::from(nums2[i2]) + dp[i1][i2][kk]);
            }
        }
    }
    dp[n1][n2][k]
}

pub fn max_score(nums1: &[i32], nums2: &[i32], k: i32) -> i64 {
    let [n1, n2] = [&nums1, &nums2].map(|v| v.len());
    let k = k as usize;
    let mut memo = vec![vec![vec![None; 1 + k]; n2]; n1];
    dfs(&nums1, &nums2, 0, 0, k, &mut memo)
}

fn dfs(
    nums1: &[i32],
    nums2: &[i32],
    i1: usize,
    i2: usize,
    k: usize,
    memo: &mut [Vec<Vec<Option<i64>>>],
) -> i64 {
    if i1 >= nums1.len() || i2 >= nums2.len() {
        return if k == 0 { return 0 } else { i64::MIN >> 2 };
    }
    if k == 0 {
        return 0;
    }
    if let Some(v) = memo[i1][i2][k] {
        return v;
    }
    let mut res = i64::from(nums1[i1]) * i64::from(nums2[i2])
        + dfs(nums1, nums2, 1 + i1, 1 + i2, k - 1, memo);
    res =
        res.max(dfs(nums1, nums2, 1 + i1, i2, k, memo))
            .max(dfs(nums1, nums2, i1, 1 + i2, k, memo));
    memo[i1][i2][k] = Some(res);
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
        assert_eq!(max_score(&[1, 3, 2], &[4, 5, 1], 2), 22);
        assert_eq!(max_score(&[-2, 0, 5], &[-3, 4, -1, 2], 2), 26);
        assert_eq!(max_score(&[-3, -2], &[1, 2], 2), -7);

        assert_eq!(with_dp(&[1, 3, 2], &[4, 5, 1], 2), 22);
        assert_eq!(with_dp(&[-2, 0, 5], &[-3, 4, -1, 2], 2), 26);
        assert_eq!(with_dp(&[-3, -2], &[1, 2], 2), -7);
    }

    #[test]
    fn test() {}
}
