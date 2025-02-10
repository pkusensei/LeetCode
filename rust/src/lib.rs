mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    map: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let map = nums2.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        Self { nums1, nums2, map }
    }

    fn add(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        self.map.entry(self.nums2[idx]).and_modify(|v| *v -= 1);
        self.nums2[idx] += val;
        *self.map.entry(self.nums2[idx]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut res = 0;
        for &n1 in self.nums1.iter() {
            if let Some(v) = self.map.get(&(tot - n1)) {
                res += v;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
