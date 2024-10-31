mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_total_distance(robot: &mut [i32], factory: &[[i32; 2]]) -> i64 {
    robot.sort_unstable();
    let mut facts: Vec<_> = factory
        .iter()
        .flat_map(|v| std::iter::repeat(v[0]).take(v[1] as usize))
        .collect();
    facts.sort_unstable();
    let (n1, n2) = (robot.len(), facts.len());

    // let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    // for v in dp.iter_mut().take(n1) {
    //     v[n2] = i64::MAX / 2;
    // }
    // // rev to ensure all future choices are considered
    // // i.e no robot is left behind
    // for (i1, &rob) in robot.iter().enumerate().rev() {
    //     for (i2, &fact) in facts.iter().enumerate().rev() {
    //         dp[i1][i2] = dp[i1][1 + i2].min(dp[1 + i1][1 + i2] + i64::from(rob.abs_diff(fact)));
    //     }
    // }
    // dp[0][0]

    let [mut curr, mut next] = [0, 1].map(|_| vec![0; 1 + n2]);
    for (i1, &rob) in robot.iter().enumerate().rev() {
        if i1 != n1 - 1 {
            next[n2] = i64::MAX / 2;
        }
        curr[n2] = i64::MAX / 2;
        for (i2, &fact) in facts.iter().enumerate().rev() {
            curr[i2] = curr[1 + i2].min(next[1 + i2] + i64::from(rob.abs_diff(fact)));
        }
        next = curr.clone();
    }
    curr[0]
    // dfs(robot, &facts, 0, 0, &mut vec![vec![-1; n2]; n1])
}

fn dfs(robot: &[i32], facts: &[i32], i1: usize, i2: usize, dp: &mut [Vec<i64>]) -> i64 {
    if i1 == robot.len() {
        return 0;
    }
    if i2 == facts.len() {
        return i64::MAX / 2;
    }
    if dp[i1][i2] > -1 {
        return dp[i1][i2];
    }
    let pick = i64::from(robot[i1].abs_diff(facts[i2])) + dfs(robot, facts, 1 + i1, 1 + i2, dp);
    let skip = dfs(robot, facts, i1, 1 + i2, dp);
    dp[i1][i2] = pick.min(skip);
    dp[i1][i2]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(minimum_total_distance(&mut [0, 4, 6], &[[2, 2], [6, 2]]), 4);
        debug_assert_eq!(minimum_total_distance(&mut [1, -1], &[[-2, 1], [2, 1]]), 2);
        debug_assert_eq!(
            minimum_total_distance(
                &mut [9, 11, 99, 101],
                &[[10, 1], [7, 1], [14, 1], [100, 1], [96, 1], [103, 1]]
            ),
            6
        );
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
