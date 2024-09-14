mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
struct Solution {
    nums: HashMap<i32, Vec<i32>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let nums: HashMap<i32, Vec<i32>> =
            nums.into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (i, num)| {
                    acc.entry(num).or_default().push(i as i32);
                    acc
                });
        Self { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        *self.nums[&target].choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let solution = Solution::new(vec![1, 2, 3, 3, 3]);
        solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
        debug_assert_eq!(solution.pick(1), 0); // It should return 0. Since in the array only nums[0] is equal to 1.
        solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
    }

    #[test]
    fn test() {}

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
