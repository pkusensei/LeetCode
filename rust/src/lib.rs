mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashMap};

// Took from https://pages.di.unipi.it/rossano/blog/2023/mosalgorithm/
pub fn subarray_majority(nums: &[i32], queries: &[[i32; 3]]) -> Vec<i32> {
    let n = nums.len();
    let qn = queries.len();
    let sqrt = 1 + n.isqrt() as i32;
    let mut res = vec![-1; qn];
    let mut track = Tracker::default();
    let mut curr_left = 0;
    let mut curr_right = 0; // exclusive
    for qi in (0..qn).sorted_unstable_by_key(|&i| [queries[i][0] / sqrt, queries[i][1]]) {
        let [left, right] = [0, 1].map(|i| queries[qi][i] as usize);
        while curr_left > left {
            curr_left -= 1;
            track.update(nums[curr_left], 1);
        }
        while curr_right <= right {
            track.update(nums[curr_right], 1);
            curr_right += 1;
        }
        while curr_left < left {
            track.update(nums[curr_left], -1);
            curr_left += 1;
        }
        while curr_right > 1 + right {
            curr_right -= 1;
            track.update(nums[curr_right], -1);
        }
        res[qi] = track.find(queries[qi][2]).unwrap_or(-1);
    }
    res
}

#[derive(Default)]
struct Tracker {
    num_freq: HashMap<i32, i32>,
    freq_nums: BTreeMap<i32, BTreeSet<i32>>,
}

impl Tracker {
    fn update(&mut self, num: i32, delta: i32) {
        let f = self.num_freq.entry(num).or_insert(0);
        *f += delta;
        let old = *f - delta;
        self.freq_nums.entry(old).or_default().remove(&num);
        self.freq_nums.entry(old + delta).or_default().insert(num);
    }

    fn find(&self, th: i32) -> Option<i32> {
        for (_, set) in self.freq_nums.range(th..).rev() {
            if let Some(&v) = set.first() {
                return Some(v);
            }
        }
        None
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
        assert_eq!(
            subarray_majority(&[1, 1, 2, 2, 1, 1], &[[0, 5, 4], [0, 3, 3], [2, 3, 2]]),
            [1, -1, 2]
        );
        assert_eq!(
            subarray_majority(
                &[3, 2, 3, 2, 3, 2, 3],
                &[[0, 6, 4], [1, 5, 2], [2, 4, 1], [3, 3, 1]]
            ),
            [3, 2, 3, 2]
        );
    }

    #[test]
    fn test() {}
}
