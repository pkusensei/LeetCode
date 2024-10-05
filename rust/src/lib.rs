mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn smallest_range(nums: &[&[i32]]) -> [i32; 2] {
    let n = nums.len();
    let mut pairs: Vec<_> = nums
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().map(move |&n| (n, i)))
        .collect();
    pairs.sort_unstable();
    let mut counts = HashMap::new();
    let mut left_i = 0;
    let mut res = [pairs[0].0, pairs.last().unwrap().0];

    for &(right, nums_i) in pairs.iter() {
        *counts.entry(nums_i).or_insert(0) += 1;
        while counts.len() == n && counts.get(&pairs[left_i].1).is_some_and(|&c| c > 1) {
            let Some(c) = counts.get_mut(&pairs[left_i].1) else {
                break;
            };
            *c -= 1;
            left_i += 1
        }
        let left = pairs[left_i].0;
        if counts.len() == n && right - left < res[1] - res[0] {
            res = [left, right];
        }
    }
    res
}

// Potentially fatser than sorting the whole merged vec
fn with_pq(nums: &[&[i32]]) -> [i32; 2] {
    let mut res = [-10_001, 10_001];
    let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = nums
        .iter()
        .enumerate()
        .map(|(nums_i, v)| (Reverse(v[0]), nums_i, 0))
        .collect();
    let mut right = nums.iter().map(|v| v[0]).max().unwrap();
    while let Some((Reverse(left), nums_i, idx)) = heap.pop() {
        if right - left < res[1] - res[0] {
            res = [left, right];
        }
        let Some(&next) = nums[nums_i].get(idx + 1) else {
            break;
        };
        heap.push((Reverse(next), nums_i, idx + 1));
        right = right.max(next);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            with_pq(&[&[4, 10, 15, 24, 26], &[0, 9, 12, 20], &[5, 18, 22, 30]]),
            [20, 24]
        );
        debug_assert_eq!(with_pq(&[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]]), [1, 1]);
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
