mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_total_distance(robot: &mut [i32], factory: &mut [[i32; 2]]) -> i64 {
    robot.sort_unstable();
    factory.sort_unstable();
    dfs(robot, factory, 0, 0, 0, &mut HashMap::new())
}

fn dfs(
    robot: &[i32],
    factory: &[[i32; 2]],
    i1: usize,
    i2: usize,
    count: i32,
    memo: &mut HashMap<(usize, usize, i32), i64>,
) -> i64 {
    if i1 >= robot.len() {
        return 0;
    }
    if i2 >= factory.len() {
        return 10_i64.pow(12);
    }
    let k = (i1, i2, count);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    // skip
    let mut res = dfs(robot, factory, i1, 1 + i2, 0, memo);
    if count < factory[i2][1] {
        let take = i64::from(robot[i1].abs_diff(factory[i2][0]))
            + dfs(robot, factory, 1 + i1, i2, 1 + count, memo);
        res = res.min(take);
    }
    memo.insert(k, res);
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(
            minimum_total_distance(&mut [0, 4, 6], &mut [[2, 2], [6, 2]]),
            4
        );
        assert_eq!(
            minimum_total_distance(&mut [1, -1], &mut [[-2, 1], [2, 1]]),
            2
        );
    }

    #[test]
    fn test() {}
}
