mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn order_of_largest_plus_sign(n: i32, mines: &[[i32; 2]]) -> i32 {
    let mines: HashSet<[usize; 2]> = mines
        .iter()
        .map(|v| [v[0] as usize, v[1] as usize])
        .collect();
    let n = n as usize;
    let mut dp = vec![vec![0; n]; n];
    // left
    let mut count = 0;
    for y in 0..n {
        count = 0;
        for x in (0..n).rev() {
            if mines.contains(&[x, y]) {
                count = 0
            } else {
                count += 1;
            }
            dp[y][x] = count;
        }
    }
    // right
    for y in 0..n {
        count = 0;
        for x in 0..n {
            if mines.contains(&[x, y]) {
                count = 0
            } else {
                count += 1;
            }
            dp[y][x] = dp[y][x].min(count);
        }
    }
    // up
    for x in 0..n {
        count = 0;
        for y in (0..n).rev() {
            if mines.contains(&[x, y]) {
                count = 0
            } else {
                count += 1;
            }
            dp[y][x] = dp[y][x].min(count);
        }
    }
    // down
    let mut res = 0;
    for x in 0..n {
        count = 0;
        for y in 0..n {
            if mines.contains(&[x, y]) {
                count = 0
            } else {
                count += 1;
            }
            dp[y][x] = dp[y][x].min(count);
            res = res.max(dp[y][x])
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(order_of_largest_plus_sign(5, &[[4, 2]]), 2);
        debug_assert_eq!(order_of_largest_plus_sign(1, &[[0, 0]]), 0);
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
}
