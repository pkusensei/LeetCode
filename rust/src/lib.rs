mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn can_cross(stones: &[i32]) -> bool {
    if stones[1] != 1 {
        return false;
    }
    let n = stones.len();
    // a vec of possible jumps
    let mut dp = vec![vec![]; n];
    dp[1].push(1);
    for i in 1..n - 1 {
        for j in i + 1..n {
            let dist = stones[j] - stones[i];
            if dp[i].iter().any(|k| (k - 1..=k + 1).contains(&dist)) {
                dp[j].push(dist)
            }
        }
    }
    !dp[n - 1].is_empty()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_cross(&[0, 1, 3, 5, 6, 8, 12, 17]));
        debug_assert!(!can_cross(&[0, 1, 2, 3, 4, 8, 9, 11]));
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
