mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn combination_sum4(nums: &[i32], target: i32) -> i32 {
    let mut dp = vec![0; 1 + target as usize];
    dp[0] = 1;
    for n in 1..=target {
        for &num in nums.iter() {
            if num <= n {
                dp[n as usize] += dp[(n - num) as usize]
            }
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(combination_sum4(&[1, 2, 3], 4), 7);
        debug_assert_eq!(combination_sum4(&[9], 3), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(combination_sum4(&[2, 1, 3], 35), 1132436852);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
