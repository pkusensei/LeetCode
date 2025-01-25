mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn best_team_score(scores: &[i32], ages: &[i32]) -> i32 {
    let mut nums: Vec<_> = scores
        .iter()
        .zip(ages.iter())
        .map(|(&s, &a)| [s, a])
        .collect();
    nums.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
    let mut dp = vec![0; nums.len()];
    let mut res = 0;
    for i1 in 0..nums.len() {
        dp[i1] = nums[i1][0];
        for i2 in 0..i1 {
            if nums[i2][0] <= nums[i1][0] {
                dp[i1] = dp[i1].max(dp[i2] + nums[i1][0]);
            }
        }
        res = res.max(dp[i1]);
    }
    res
    // dfs(&nums, 0, 0, 0, &mut HashMap::new())
}

// [score, age]
fn dfs(
    nums: &[[i32; 2]],
    idx: usize,
    cs: i32,
    ca: i32,
    memo: &mut HashMap<(usize, i32, i32), i32>,
) -> i32 {
    if idx >= nums.len() {
        return 0;
    }
    let key = (idx, cs, ca);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let skip = dfs(nums, 1 + idx, cs, ca, memo);
    let [score, age] = nums[idx];
    let take = if cs <= score {
        score + dfs(nums, 1 + idx, score, age, memo)
    } else {
        0
    };
    let res = skip.max(take);
    memo.insert(key, res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(best_team_score(&[1, 3, 5, 10, 15], &[1, 2, 3, 4, 5]), 34);
        assert_eq!(best_team_score(&[4, 5, 6, 5], &[2, 1, 2, 1]), 16);
        assert_eq!(best_team_score(&[1, 2, 3, 5], &[8, 9, 10, 1]), 6);
    }

    #[test]
    fn test() {}

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
