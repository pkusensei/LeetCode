mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn job_scheduling(start_time: &[i32], end_time: &[i32], profit: &[i32]) -> i32 {
    let mut jobs: Vec<_> = start_time
        .iter()
        .zip(end_time.iter())
        .zip(profit.iter())
        .map(|((s, e), p)| [*s, *e, *p])
        .collect();
    jobs.sort_unstable_by_key(|v| v[1]);
    let n = jobs.len();
    let mut dp = vec![0; 1 + n];
    for (idx, &[s, _e, p]) in jobs.iter().enumerate() {
        let i = jobs.partition_point(|v| v[1] <= s);
        // skip current item
        // vs
        // pick item, and prev item(s)
        dp[1 + idx] = dp[idx].max(p + dp[i])
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            job_scheduling(&[1, 2, 3, 3], &[3, 4, 5, 6], &[50, 10, 40, 70]),
            120
        );
        assert_eq!(
            job_scheduling(&[1, 2, 3, 4, 6], &[3, 5, 10, 6, 9], &[20, 20, 100, 70, 60]),
            150
        );
        assert_eq!(job_scheduling(&[1, 1, 1], &[2, 3, 4], &[5, 6, 4]), 6);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
