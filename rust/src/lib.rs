mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mincost_tickets(days: &[i32], costs: [i32; 3]) -> i32 {
    let n = *days.last().unwrap_or(&1);
    let set: std::collections::HashSet<i32> = days.iter().copied().collect();
    let mut dp = Vec::with_capacity(1 + n as usize);
    dp.push(0); // day 0 => no cost
    for day in 1..=n {
        if set.contains(&day) {
            let d1 = costs[0] + dp[day as usize - 1];
            let d2 = if day >= 7 {
                costs[1] + dp[day as usize - 7]
            } else {
                costs[1]
            };
            let d3 = if day >= 30 {
                costs[2] + dp[day as usize - 30]
            } else {
                costs[2]
            };
            dp.push(d1.min(d2).min(d3));
        } else {
            dp.push(*dp.last().unwrap_or(&0));
        }
    }
    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(mincost_tickets(&[1, 4, 6, 7, 8, 20], [2, 7, 15]), 11);
        debug_assert_eq!(
            mincost_tickets(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], [2, 7, 15]),
            17
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
