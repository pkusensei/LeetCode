mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn tallest_billboard(rods: &[i32]) -> i32 {
    // taller - shorter <-> taller
    let mut dp = HashMap::from([(0, 0)]);
    for &num in rods.iter() {
        let mut curr = dp.clone();
        for (&diff, &tall) in dp.iter() {
            let short = tall - diff;

            let new_tall = curr.get(&(diff + num)).copied().unwrap_or(0);
            curr.insert(diff + num, new_tall.max(tall + num));

            let new_diff = (short + num - tall).abs();
            let new_tall = tall.max(short + num);
            let v = curr.get(&new_diff).copied().unwrap_or(0).max(new_tall);
            curr.insert(new_diff, v);
        }
        dp = curr;
    }
    *dp.get(&0).unwrap_or(&0)
}

fn meet_in_middle(nums: &[i32]) -> i32 {
    fn build_half(nums: &[i32]) -> HashMap<i32, i32> {
        let mut states = HashSet::from([(0, 0)]);
        for &num in nums.iter() {
            let mut curr = HashSet::new();
            for &(a, b) in states.iter() {
                curr.insert((a + num, b));
                curr.insert((a, b + num));
            }
            states.extend(curr);
        }
        states
            .into_iter()
            .fold(HashMap::new(), |mut acc, (left, right)| {
                let val = acc.get(&(left - right)).copied().unwrap_or(0).max(left);
                acc.insert(left - right, val);
                acc
            })
    }

    let n = nums.len();
    let first = build_half(&nums[..n / 2]);
    let second = build_half(&nums[n / 2..]);
    let mut res = 0;
    for (diff, v1) in first.iter() {
        if let Some(v2) = second.get(&(-diff)) {
            res = res.max(v1 + v2);
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
        debug_assert_eq!(tallest_billboard(&[1, 2, 3, 6]), 6);
        debug_assert_eq!(tallest_billboard(&[1, 2, 3, 4, 5, 6]), 10);
        debug_assert_eq!(tallest_billboard(&[1, 2]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            tallest_billboard(&[
                102, 101, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100
            ]),
            900
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
