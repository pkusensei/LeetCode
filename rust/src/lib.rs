mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_2d_dp_opt_space(nums1: &[i32], nums2: &[i32]) -> i32 {
    // Since in 2d table, we use only
    // prev row: [i1-1][i2-1], [i1-1][i2]
    // curr row: [i1][i2-1]
    // Only 2 rows are involved
    let [_n1, n2] = [nums1.len(), nums2.len()];
    // +1 for padding: dp[0] used as sentinal
    let mut dp = vec![i32::MIN; 1 + n2];
    for (_i1, val1) in nums1.iter().enumerate() {
        let mut curr = vec![i32::MIN; 1 + n2];
        for (i2, val2) in nums2.iter().enumerate() {
            curr[1 + i2] =
                    // Greedily take previous result
                    (val1 * val2 + dp[i2].max(0))
                        // Skip [i1]
                        .max(dp[1 + i2])
                        // Skip [i2]
                        .max(curr[i2]);
        }
        dp = curr;
    }
    dp[n2]
}

pub fn with_2d_dp(nums1: &[i32], nums2: &[i32]) -> i32 {
    let [n1, n2] = [nums1.len(), nums2.len()];
    let mut dp = vec![vec![0; n2]; n1];
    // Simplest case: n1==n2==1
    dp[0][0] = nums1[0] * nums2[0];
    // Fill first row: n2==1
    for (i, val) in nums1.iter().enumerate().skip(1) {
        dp[i][0] = (val * nums2[0]).max(dp[i - 1][0]);
    }
    // Fill first col: n1==1
    for (i, val) in nums2.iter().enumerate().skip(1) {
        dp[0][i] = (nums1[0] * val).max(dp[0][i - 1]);
    }
    for (i1, val1) in nums1.iter().enumerate().skip(1) {
        for (i2, val2) in nums2.iter().enumerate().skip(1) {
            dp[i1][i2] =
                    // Greedily take any previous result
                    (dp[i1 - 1][i2 - 1].max(0) + val1 * val2)
                    // Skip [i1]
                    .max(dp[i1 - 1][i2])
                    // Skip [i2]
                    .max(dp[i1][i2 - 1]);
        }
    }
    dp[n1 - 1][n2 - 1]
}

pub fn max_dot_product(nums1: &[i32], nums2: &[i32]) -> i32 {
    let n1 = nums1.len();
    let n2 = nums2.len();
    dfs(&nums1, &nums2, 0, 0, &mut vec![vec![i32::MIN >> 1; n2]; n1])
}

fn dfs(nums1: &[i32], nums2: &[i32], i1: usize, i2: usize, memo: &mut [Vec<i32>]) -> i32 {
    if i1 >= nums1.len() || i2 >= nums2.len() {
        return i32::MIN >> 1;
    }
    if memo[i1][i2] > (i32::MIN >> 1) {
        return memo[i1][i2];
    }
    let res = (nums1[i1] * nums2[i2])
        .max(dfs(nums1, nums2, 1 + i1, 1 + i2, memo))
        .max(dfs(nums1, nums2, 1 + i1, i2, memo))
        .max(dfs(nums1, nums2, i1, 1 + i2, memo))
        .max(nums1[i1] * nums2[i2] + dfs(nums1, nums2, 1 + i1, 1 + i2, memo));
    memo[i1][i2] = res;
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
        assert_eq!(max_dot_product(&[2, 1, -2, 5], &[3, 0, -6]), 18);
        assert_eq!(max_dot_product(&[3, -2], &[2, -6, 7]), 21);
        assert_eq!(max_dot_product(&[-1, -1], &[1, 1]), -1);

        assert_eq!(with_2d_dp(&[2, 1, -2, 5], &[3, 0, -6]), 18);
        assert_eq!(with_2d_dp(&[3, -2], &[2, -6, 7]), 21);
        assert_eq!(with_2d_dp(&[-1, -1], &[1, 1]), -1);

        assert_eq!(with_2d_dp_opt_space(&[2, 1, -2, 5], &[3, 0, -6]), 18);
        assert_eq!(with_2d_dp_opt_space(&[3, -2], &[2, -6, 7]), 21);
        assert_eq!(with_2d_dp_opt_space(&[-1, -1], &[1, 1]), -1);
    }

    #[test]
    fn test() {}
}
