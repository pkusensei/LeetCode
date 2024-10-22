mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let r = query_row as usize;
    let mut dp = vec![vec![0.0; 102]; 2 + r];
    dp[0][0] = f64::from(poured);
    for row in 0..=r {
        for col in 0..=row {
            let v = (dp[row][col] - 1.0) * 0.5;
            if v > 0.0 {
                dp[1 + row][col] += v;
                dp[1 + row][1 + col] += v;
            }
        }
    }
    dp[r][query_glass as usize].min(1.0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(champagne_tower(1, 1, 1), 0.00000);
        debug_assert_eq!(champagne_tower(2, 1, 1), 0.50000);
        debug_assert_eq!(champagne_tower(100_000_009, 33, 17), 1.00000);
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
