mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(houses: &[i32], cost: &[&[i32]], m: i32, n: i32, target: i32) -> i32 {
    let [m, n] = [m, n].map(|v| v as usize);
    // m, n, target
    let mut memo = vec![vec![vec![None; 1 + target as usize]; 1 + n]; m];
    dfs(houses, cost, m, target, 0, 0, &mut memo).unwrap_or(-1)
}

fn dfs(
    houses: &[i32],
    cost: &[&[i32]],
    m: usize,
    target: i32,
    idx: usize,
    prev: usize, // color of previous house
    memo: &mut [Vec<Vec<Option<i32>>>],
) -> Option<i32> {
    if target < 0 {
        return None;
    }
    if idx >= m {
        return if target == 0 { Some(0) } else { None };
    }
    if let Some(v) = memo[idx][prev][target as usize] {
        return if v == -1 { None } else { Some(v) };
    }
    if houses[idx] > 0 {
        return dfs(
            houses,
            cost,
            m,
            target - i32::from(houses[idx] as usize != prev),
            1 + idx,
            houses[idx] as usize,
            memo,
        );
    }
    let mut res = i32::MAX;
    // 1+cidx=>color
    // c=>cost
    for (cidx, &c) in cost[idx].iter().enumerate() {
        if let Some(v) = dfs(
            houses,
            cost,
            m,
            target - i32::from(1 + cidx != prev),
            1 + idx,
            1 + cidx,
            memo,
        ) {
            res = res.min(c + v);
        }
    }
    if res == i32::MAX {
        memo[idx][prev][target as usize] = Some(-1);
        None
    } else {
        memo[idx][prev][target as usize] = Some(res);
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            min_cost(
                &[0, 0, 0, 0, 0],
                &[&[1, 10], &[10, 1], &[10, 1], &[1, 10], &[5, 1]],
                5,
                2,
                3
            ),
            9
        );
        assert_eq!(
            min_cost(
                &[0, 2, 1, 2, 0],
                &[&[1, 10], &[10, 1], &[10, 1], &[1, 10], &[5, 1]],
                5,
                2,
                3
            ),
            11
        );
        assert_eq!(
            min_cost(
                &[3, 1, 2, 3],
                &[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1], &[1, 1, 1]],
                4,
                3,
                3
            ),
            -1
        )
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
