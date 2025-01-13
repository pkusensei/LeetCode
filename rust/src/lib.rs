mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_iii(stone_value: &[i32]) -> String {
    let n = stone_value.len();
    let mut dp = [0; 4];
    for i in (0..n).rev() {
        dp[i % 4] = stone_value[i] - dp[(1 + i) % 4];
        if i + 2 <= n {
            let take_two = stone_value[i] + stone_value[i + 1] - dp[(2 + i) % 4];
            dp[i % 4] = dp[i % 4].max(take_two);
        }
        if i + 3 <= n {
            let take_three = (0..3).map(|k| stone_value[i + k]).sum::<i32>() - dp[(3 + i) % 4];
            dp[i % 4] = dp[i % 4].max(take_three);
        }
    }
    match dp[0].cmp(&0) {
        // match dfs(stone_value, 0, 0, &mut vec![[None; 2]; n]).cmp(&0) {
        std::cmp::Ordering::Less => "Bob".into(),
        std::cmp::Ordering::Equal => "Tie".into(),
        std::cmp::Ordering::Greater => "Alice".into(),
    }
}

fn dfs(nums: &[i32], idx: usize, turn: usize, memo: &mut [[Option<i32>; 2]]) -> i32 {
    if idx >= nums.len() {
        return 0;
    }
    if let Some(v) = memo[idx][turn] {
        return v;
    }
    let mut curr = nums[idx];
    let mut res = dfs(nums, 1 + idx, 1 - turn, memo) + if turn == 0 { curr } else { -curr };
    for i in [idx + 1, idx + 2] {
        if let Some(&v) = nums.get(i) {
            curr += v;
            if turn == 0 {
                res = res.max(curr + dfs(nums, 1 + i, 1 - turn, memo));
            } else {
                res = res.min(-curr + dfs(nums, 1 + i, 1 - turn, memo));
            }
        }
    }
    memo[idx][turn] = Some(res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(stone_game_iii(&[1, 2, 3, 7]), "Bob");
        assert_eq!(stone_game_iii(&[1, 2, 3, -9]), "Alice");
        assert_eq!(stone_game_iii(&[1, 2, 3, 6]), "Tie");
    }

    #[test]
    fn test() {}

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
