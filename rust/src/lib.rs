mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// For matrix like this
// 0 0 0 1   transforms  0 0 0 1
// 0 0 1 1    into =>    0 0 1 2
// 0 1 1 1               0 1 2 3
// ............................^.
// Stand on [r=2, c=3] and look up
// [r=2] num==3, the current submat [1 2 3] counts as 3
// [r=1] min(3,2)==2, it forms 2 submats with height==2
// [r=0] simiarly, only vertical [1 2 3] counts
pub fn num_submat(mat: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(mat);
    let mut dp = vec![vec![0; cols]; rows];
    let mut res = 0;
    for (r, row) in mat.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val == 0 {
                continue;
            }
            dp[r][c] = 1 + if c == 0 { 0 } else { dp[r][c - 1] };
            let mut curr = dp[r][c];
            for i in (0..=r).rev() {
                curr = curr.min(dp[i][c]);
                res += curr;
            }
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
        assert_eq!(num_submat(&[&[1, 0, 1], &[1, 1, 0], &[1, 1, 0]]), 13);
        assert_eq!(
            num_submat(&[&[0, 1, 1, 0], &[0, 1, 1, 1], &[1, 1, 1, 0]]),
            24
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
