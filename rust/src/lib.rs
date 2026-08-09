mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    if n <= 2 {
        return piles.iter().sum();
    }
    let mut suffix = piles.to_vec();
    for i in (0..n - 1).rev() {
        suffix[i] += suffix[1 + i];
    }
    let mut dp = vec![vec![0; n]; n];
    for idx in (0..n).rev() {
        for m in 1..n {
            if idx + 2 * m >= n {
                dp[idx][m] = suffix[idx];
            } else {
                for x in 1..=2 * m {
                    let v = dp[idx + x][x.max(m)];
                    dp[idx][m] = dp[idx][m].max(suffix[idx] - v);
                }
            }
        }
    }
    dp[0][1]
    // dfs(&suffix, 0, 1)
}

fn dfs(nums: &[i32], idx: usize, m: usize) -> i32 {
    let n = nums.len();
    if idx + 2 * m >= n {
        return nums[idx];
    }
    let mut res = 0;
    for x in 1..=2 * m {
        let v = dfs(nums, idx + x, m.max(x));
        res = res.max(nums[idx] - v);
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
    fn basics() {}

    #[test]
    fn test() {}
}
