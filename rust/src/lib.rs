mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn connect_two_groups(cost: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(cost);
    dfs(cost, 0, 0, &mut vec![vec![-1; 1 << cols]; rows])
}

// count/id of left group
// mask of right group
// size(left)>=size(right)
// Try connect every node on left with min cost first
// Then connect any right node left
fn dfs(cost: &[&[i32]], left: usize, mask: usize, memo: &mut [Vec<i32>]) -> i32 {
    let [rows, cols] = get_dimensions(cost);
    if left >= rows {
        let mut res = 0;
        for c in 0..cols {
            let val = (0..rows)
                .map(|r| cost[r][c])
                .min()
                .map(|val| val * i32::from((mask & (1 << c)) == 0))
                .unwrap_or(0);
            res += val;
        }
        return res;
    }
    if memo[left][mask] > -1 {
        return memo[left][mask];
    }
    let mut res = i32::MAX;
    for c in 0..cols {
        res = res.min(cost[left][c] + dfs(cost, 1 + left, mask | (1 << c), memo));
    }
    memo[left][mask] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(connect_two_groups(&[&[15, 96], &[36, 2]]), 17);
        assert_eq!(connect_two_groups(&[&[1, 3, 5], &[4, 1, 1], &[1, 5, 3]]), 4);
        assert_eq!(
            connect_two_groups(&[&[2, 5, 1], &[3, 4, 7], &[8, 1, 2], &[6, 2, 4], &[3, 8, 8]]),
            10
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
