mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<[i32; 2]>) -> i64 {
    robot.sort_unstable();
    factory.sort_unstable();
    let facts = factory
        .iter()
        .flat_map(|f| repeat_n(f[0], f[1] as usize))
        .collect_vec();
    let (nr, nf) = (robot.len(), facts.len());
    let mut dp = vec![vec![i64::MAX >> 2; 1 + nf]; 1 + nr];
    // dp[nr].fill(0); // no robot left
    // for i1 in (0..nr).rev() {
    //     for i2 in (i1..nf).rev() {
    //         dp[i1][i2] =
    //             dp[i1][1 + i2].min(i64::from(robot[i1].abs_diff(facts[i2])) + dp[1 + i1][1 + i2]);
    //     }
    // }
    // dp[0][0]
    dp[0].fill(0);
    for i1 in 0..nr {
        for i2 in i1..nf {
            dp[1 + i1][1 + i2] =
                dp[1 + i1][i2].min(i64::from(robot[i1].abs_diff(facts[i2])) + dp[i1][i2]);
        }
    }
    dp[nr][nf]
    // dfs(&robot, &facts, nr, nf)
}

fn dfs1(robot: &[i32], facts: &[i32], i1: usize, i2: usize) -> i64 {
    if i1 == robot.len() {
        return 0;
    }
    if i2 == facts.len() {
        return i64::MAX >> 2;
    }
    let skip = dfs(robot, facts, i1, i2 + 1);
    let take = i64::from(robot[i1].abs_diff(facts[i2])) + dfs(robot, facts, i1 + 1, i2 + 1);
    skip.min(take)
}

fn dfs(robot: &[i32], facts: &[i32], i1: usize, i2: usize) -> i64 {
    if i1 == 0 {
        return 0;
    }
    if i2 == 0 {
        return i64::MAX >> 2;
    }
    let skip = dfs(robot, facts, i1, i2 - 1);
    let take = i64::from(robot[i1 - 1].abs_diff(facts[i2 - 1])) + dfs(robot, facts, i1 - 1, i2 - 1);
    skip.min(take)
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
    fn test() {
        assert_eq!(
            minimum_total_distance(
                vec![9, 11, 99, 101],
                vec![[10, 1], [7, 1], [14, 1], [100, 1], [96, 1], [103, 1]]
            ),
            6
        );
    }
}
