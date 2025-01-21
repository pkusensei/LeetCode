mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// 0 0 0 0 # # #
// # # # 0 0 0 0
// Robot2 can take either all #'s on top row or all on bottom row
// Thus for each i, curr = max(top_prefix[1+i..n], down_prefix[i])
// Then min(all curr)
pub fn grid_game(grid: [&[i32]; 2]) -> i64 {
    let n = grid[0].len();
    let [mut top, mut down] = [0, 1].map(|_| Vec::with_capacity(n));
    for idx in 0..n {
        top.push(i64::from(grid[0][idx]) + top.last().unwrap_or(&0));
        down.push(i64::from(grid[1][idx]) + down.last().unwrap_or(&0));
    }
    let mut res = i64::MAX;
    for idx in 0..n {
        let curr = (top[n - 1] - top[idx]).max(if idx > 0 { down[idx - 1] } else { 0 });
        res = res.min(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(grid_game([&[2, 5, 4], &[1, 5, 1]]), 4);
        assert_eq!(grid_game([&[3, 3, 1], &[8, 5, 2]]), 4);
        assert_eq!(grid_game([&[1, 3, 1, 15], &[1, 3, 3, 1]]), 7);
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
