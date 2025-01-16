mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_dot_product(nums1: &[i32], nums2: &[i32]) -> i32 {
    let [n1, n2] = [nums1, nums2].map(|v| v.len());
    // dfs(nums1, nums2, 0, 0, &mut vec![vec![None; n2]; n1]).unwrap()
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    let mut res = i32::MIN;
    for (i1, v1) in nums1.iter().enumerate() {
        for (i2, v2) in nums2.iter().enumerate() {
            res = res.max(v1 * v2);
            dp[1 + i1][1 + i2] = dp[1 + i1][1 + i2]
                .max(dp[i1][1 + i2])
                .max(dp[1 + i1][i2])
                .max(dp[i1][i2] + v1 * v2);
        }
    }
    if res >= 0 {
        // initial value in dp is set to 0
        // max res is max (v1*v2)
        // this check ensures that init 0 does not offset negative max values
        res.max(dp[n1][n2])
    } else {
        res
    }
}

fn dfs(
    nums1: &[i32],
    nums2: &[i32],
    i1: usize,
    i2: usize,
    memo: &mut [Vec<Option<i32>>],
) -> Option<i32> {
    if i1 >= nums1.len() || i2 >= nums2.len() {
        return None;
    }
    if let Some(v) = memo[i1][i2] {
        return Some(v);
    }
    let mut res = Some(nums1[i1] * nums2[i2]);
    let pick = nums1[i1] * nums2[i2] + dfs(nums1, nums2, 1 + i1, 1 + i2, memo).unwrap_or(0);
    let skip_i2 = dfs(nums1, nums2, i1, 1 + i2, memo);
    let skip_i1 = dfs(nums1, nums2, 1 + i1, i2, memo);
    let skip = dfs(nums1, nums2, 1 + i1, 1 + i2, memo);
    res = res.max(Some(pick)).max(skip_i1).max(skip_i2).max(skip);
    memo[i1][i2] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_dot_product(&[2, 1, -2, 5], &[3, 0, -6]), 18);
        assert_eq!(max_dot_product(&[3, -2], &[2, -6, 7]), 21);
        assert_eq!(max_dot_product(&[-1, -1], &[1, 1]), -1);
    }

    #[test]
    fn test() {
        assert_eq!(max_dot_product(&[-5, -1, -2], &[3, 3, 5, 5]), -3);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
