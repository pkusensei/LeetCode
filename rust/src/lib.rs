mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{collections::BTreeMap, sync::LazyLock};

#[allow(unused_imports)]
use helper::*;

pub fn prime_subarray(nums: &[i32], k: i32) -> i32 {
    let mut res = 0;
    let mut left = 0;
    let mut ms = MultiSet::default();
    let [mut prev, mut curr] = [0; 2];
    for (right, &num) in nums.iter().enumerate() {
        if S[num as usize] {
            prev = curr;
            curr = right;
            ms.insert(num);
        }
        while ms.gap().is_some_and(|v| v > k) {
            ms.remove(nums[left]);
            left += 1;
        }
        if ms.count >= 2 {
            res += prev - left + 1;
        }
    }
    res as i32
}

const MAX: usize = 50_001;
static S: LazyLock<[bool; MAX]> = LazyLock::new(|| {
    let mut res = [true; MAX];
    res[..2].fill(false);
    for p in 2..=MAX.isqrt() {
        if res[p] {
            for val in (p * p..MAX).step_by(p) {
                res[val] = false;
            }
        }
    }
    res
});

#[derive(Default)]
struct MultiSet {
    map: BTreeMap<i32, i32>,
    count: i32,
}

impl MultiSet {
    fn insert(&mut self, num: i32) {
        *self.map.entry(num).or_insert(0) += 1;
        self.count += 1;
    }

    fn remove(&mut self, num: i32) {
        if let Some(v) = self.map.get_mut(&num) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(&num);
            }
            self.count -= 1;
        }
    }

    fn gap(&self) -> Option<i32> {
        self.map
            .keys()
            .next()
            .zip(self.map.keys().last())
            .map(|(a, b)| b - a)
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {
        assert_eq!(prime_subarray(&[1, 2, 3], 1), 2);
        assert_eq!(prime_subarray(&[2, 3, 5, 7], 3), 4);
    }

    #[test]
    fn test() {}
}
