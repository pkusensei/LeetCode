mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
struct Solution {
    m: i32,
    n: i32,
    record: BTreeSet<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            m,
            n,
            record: BTreeSet::new(),
            rng: rand::thread_rng(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut area = self
            .rng
            .gen_range(0..self.m * self.n - self.record.len() as i32);
        area += self.record.iter().filter(|&&n| n < area).count() as i32;
        while self.record.contains(&area) {
            area += 1
        }
        self.record.insert(area);
        vec![area % self.m, area / self.m]
    }

    fn reset(&mut self) {
        self.record.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut solution = Solution::new(3, 1);
        solution.flip(); // return [1, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
        solution.flip(); // return [2, 0], Since [1,0] was returned, [2,0] and [0,0]
        solution.flip(); // return [0, 0], Based on the previously returned indices, only [0,0] can be returned.
        solution.reset(); // All the values are reset to 0 and can be returned.
        solution.flip(); // return [2, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
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
