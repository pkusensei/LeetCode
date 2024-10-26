mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn num_factored_binary_trees(arr: &mut [i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    arr.sort_unstable();
    let mut dp: HashMap<i64, i64> = HashMap::new();
    for &num in arr.iter() {
        let num = i64::from(num);
        let mut count = 1;
        for &left in dp.keys() {
            if num % left == 0 {
                let right = num / left;
                if let Some(&v) = dp.get(&right) {
                    // For 16 = 2*8 4*4
                    // [2,8] and [8,2] are both counted
                    // but each key is visited only once
                    // i.e [4,4] is counted only once
                    count = (count + v * dp[&left]) % MOD;
                }
            }
        }
        dp.insert(num, count);
    }
    dp.into_values().fold(0, |acc, v| (acc + v) % MOD) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_factored_binary_trees(&mut [2, 4]), 3);
        debug_assert_eq!(num_factored_binary_trees(&mut [2, 4, 5, 10]), 7);
        debug_assert_eq!(num_factored_binary_trees(&mut [2, 4, 8, 16]), 23);
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
