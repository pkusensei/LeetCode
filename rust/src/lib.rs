mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let (n, k) = (n as usize, k as usize);
    // dfs(n, k, &mut vec![vec![-1; 1 + n]; 1 + k])
    let mut dp = vec![0; 1 + k];
    let mut moves = 0;
    while dp[k] < n {
        // rev to avoid duplicate counting ??
        for i in (1..=k).rev() {
            // With one more move, the highest floor reachable is
            // dp[m][i] = 1 + dp[m-1][i-1] + dp[m-1][i]
            // 1 + prev case when egg breaks + prev when egg is intact
            dp[i] = 1 + dp[i - 1] + dp[i];
        }
        moves += 1;
    }
    moves
}

fn dfs(n: usize, k: usize, dp: &mut [Vec<i32>]) -> i32 {
    if n < 2 || k == 1 {
        return n as i32;
    }
    if dp[k][n] > -1 {
        return dp[k][n];
    }
    let (mut left, mut right) = (1, n);
    let mut res = i32::MAX;
    while left <= right {
        let mid = left + (right - left) / 2;
        let low = dfs(mid - 1, k - 1, dp); // egg broken
        let high = dfs(n - mid, k, dp);
        res = res.min(1 + low.max(high));
        if low < high {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    dp[k][n] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(super_egg_drop(1, 2), 2);
        debug_assert_eq!(super_egg_drop(2, 6), 3);
        debug_assert_eq!(super_egg_drop(3, 14), 4);
    }

    #[test]
    fn test() {
        debug_assert_eq!(super_egg_drop(2, 2), 2);
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
