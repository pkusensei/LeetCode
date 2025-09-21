mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn min_split_merge(nums1: &[i32], nums2: &[i32]) -> i32 {
    let map = nums1.iter().fold(HashMap::new(), |mut acc, &v| {
        let n = acc.len() as i32;
        acc.entry(v).or_insert(n);
        acc
    });
    let start = to_bits(&map, nums1);
    let target = to_bits(&map, nums2);
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    while let Some((curr, step)) = queue.pop_front() {
        if curr == target {
            return step;
        }
        for next in generate(nums1.len(), curr, &mut seen) {
            queue.push_back((next, 1 + step));
        }
    }
    -1
}

const WIDTH: i32 = 3; // 0b111

fn generate(n: usize, mut bits: i32, seen: &mut HashSet<i32>) -> Vec<i32> {
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        nums.push(bits & 0b111);
        bits >>= WIDTH;
    }
    nums.reverse();
    let mut res = vec![];
    for left in 0..n {
        for right in left..n {
            let mut curr = nums[..left]
                .iter()
                .chain(&nums[1 + right..])
                .copied()
                .collect_vec();
            let sub = &nums[left..=right];
            for i in 0..curr.len() {
                let mut temp = curr.clone();
                temp.splice(i..i, sub.iter().copied());
                let mask = temp.iter().fold(0, |acc, v| (acc << WIDTH) | v);
                if seen.insert(mask) {
                    res.push(mask);
                }
            }
            curr.extend_from_slice(sub);
            let mask = curr.iter().fold(0, |acc, v| (acc << WIDTH) | v);
            if seen.insert(mask) {
                res.push(mask);
            }
        }
    }
    res
}

// len(nums)<=6
fn to_bits(map: &HashMap<i32, i32>, nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, v| (acc << WIDTH) | map[v])
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
        assert_eq!(min_split_merge(&[3, 1, 2], &[1, 2, 3]), 1);
        assert_eq!(min_split_merge(&[1, 1, 2, 3, 4, 5], &[5, 4, 3, 2, 1, 1]), 3)
    }

    #[test]
    fn test() {
        assert_eq!(min_split_merge(&[-17, -31], &[-31, -17]), 1);
    }
}
