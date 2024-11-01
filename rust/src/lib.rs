mod dsu;
mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: &[[i32; 2]]) -> i32 {
    let mut curr = start_fuel;
    if curr >= target {
        return 0;
    }
    let mut heap = BinaryHeap::new();
    let mut res = 0;
    for station in stations.iter() {
        let (pos, fuel) = (station[0], station[1]);
        // Has to reach pos before adding its fuel to heap
        while curr < pos {
            let Some(fuel) = heap.pop() else { return -1 };
            curr += fuel;
            res += 1;
        }
        heap.push(fuel);
    }
    if target <= curr {
        res
    } else {
        while curr < target {
            let Some(fuel) = heap.pop() else { return -1 };
            curr += fuel;
            res += 1;
        }
        res
    }
}

fn with_dp(target: i32, start_fuel: i32, stations: &[[i32; 2]]) -> i32 {
    let n = stations.len();
    let mut dp = vec![0; 1 + n];
    dp[0] = start_fuel;
    for (idx, &[pos, fuel]) in stations.iter().enumerate() {
        for t in (0..=idx).rev() {
            if pos <= dp[t] {
                dp[t + 1] = dp[t + 1].max(fuel + dp[t]);
            }
        }
    }
    dp.into_iter()
        .enumerate()
        .find_map(|(i, v)| if target <= v { Some(i as i32) } else { None })
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_dp(1, 1, &[]), 0);
        debug_assert_eq!(with_dp(100, 1, &[[10, 100]]), -1);
        debug_assert_eq!(
            with_dp(100, 10, &[[10, 60], [20, 30], [30, 30], [60, 40]]),
            2
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            with_dp(
                1000,
                299,
                &[
                    [13, 21],
                    [26, 115],
                    [100, 47],
                    [225, 99],
                    [299, 141],
                    [444, 198],
                    [608, 190],
                    [636, 157],
                    [647, 255],
                    [841, 123]
                ]
            ),
            4
        );
    }

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
