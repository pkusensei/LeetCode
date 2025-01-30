mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_height(cuboids: &mut [[i32; 3]]) -> i32 {
    let n = cuboids.len();
    for c in cuboids.iter_mut() {
        c.sort_unstable();
    }
    cuboids.sort_unstable();
    let mut dp: Vec<i32> = cuboids.iter().map(|c| c[2]).collect();
    for i1 in 1..n {
        for i2 in 0..i1 {
            if cuboids[i2]
                .iter()
                .zip(cuboids[i1].iter())
                .all(|(a, b)| a <= b)
            {
                dp[i1] = dp[i1].max(cuboids[i1][2] + dp[i2])
            }
        }
    }
    dp.into_iter().max().unwrap_or(cuboids[n - 1][2])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            max_height(&mut [[50, 45, 20], [95, 37, 53], [45, 23, 12]]),
            190
        );
        assert_eq!(max_height(&mut [[38, 25, 45], [76, 35, 3]]), 76);
        assert_eq!(
            max_height(&mut [
                [7, 11, 17],
                [7, 17, 11],
                [11, 7, 17],
                [11, 17, 7],
                [17, 7, 11],
                [17, 11, 7]
            ]),
            102
        );
    }

    #[test]
    fn test() {}

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
