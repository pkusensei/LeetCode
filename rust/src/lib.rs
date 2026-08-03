mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_iii(stone_value: &[i32]) -> String {
    let n = stone_value.len();
    let mut dp = vec![i32::MIN; n];
    // Could push in dp[n] = 0;
    for idx in (0..n).rev() {
        for next in idx + 1..=(idx + 3).min(n) {
            dp[idx] = dp[idx]
                .max(stone_value[idx..next].iter().sum::<i32>() - dp.get(next).unwrap_or(&0));
        }
    }
    match dp[0].cmp(&0) {
        // match dfs(&stone_value, 0).cmp(&0) {
        std::cmp::Ordering::Less => "Bob",
        std::cmp::Ordering::Equal => "Tie",
        std::cmp::Ordering::Greater => "Alice",
    }
    .to_string()
}

fn dfs(nums: &[i32], idx: usize) -> i32 {
    let n = nums.len();
    if idx >= n {
        return 0;
    }
    let mut res = i32::MIN;
    for i in idx + 1..=(idx + 3).min(n) {
        let curr = nums[idx..i].iter().sum::<i32>() - dfs(nums, i);
        res = res.max(curr)
    }
    res
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
    fn basics() {
        assert_eq!(stone_game_iii(&[1, 2, 3, 6]), "Tie")
    }

    #[test]
    fn test() {}
}
