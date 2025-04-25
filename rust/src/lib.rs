mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn earliest_second_to_mark_indices(nums: &[i32], change_indices: &[i32]) -> i32 {
    let n = nums.len();
    let m = change_indices.len();
    if m < n {
        return -1;
    }
    let mut earliest = HashMap::new();
    for (i, &val) in change_indices.iter().enumerate() {
        earliest.entry(val).or_insert(i);
    }
    let mut left = 0;
    let mut right = m - 1;
    let mut res = -1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if check(nums, change_indices, &earliest, mid) {
            res = 1 + mid as i32;
            let Some(v) = mid.checked_sub(1) else {
                break;
            };
            right = v;
        } else {
            left = mid + 1;
        }
    }
    res
}

fn check(nums: &[i32], change_indices: &[i32], earliest: &HashMap<i32, usize>, mid: usize) -> bool {
    let mut sum: i64 = nums.iter().map(|&v| i64::from(v)).sum();
    let mut heap = BinaryHeap::new();
    let mut marked = 1; // available operations
    for (i, &val) in change_indices[..mid].iter().enumerate().rev() {
        let idx = val as usize - 1;
        if nums[idx] == 0 {
            marked += 1;
        } else if earliest.get(&val).is_some_and(|&ei| ei == i) {
            // By this time, [idx] has to be zeroed
            // Save it for optimal reduction
            heap.push(Reverse(nums[idx]));
            marked -= 1;
            sum -= i64::from(nums[idx]);
            if marked < 0 {
                // Back to 1; 2 ops, zeroing and marking
                marked += 2;
                // This records the ops of decrementing by 1
                sum += i64::from(heap.pop().unwrap_or_default().0);
            }
        } else {
            marked += 1;
        }
    }
    (nums.len() + heap.len()) as i64 + sum <= 1 + mid as i64
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
    fn basics() {
        assert_eq!(
            earliest_second_to_mark_indices(&[3, 2, 3], &[1, 3, 2, 2, 2, 2, 3],),
            6
        );
        assert_eq!(
            earliest_second_to_mark_indices(&[0, 0, 1, 2], &[1, 2, 1, 2, 1, 2, 1, 2]),
            7
        );
        assert_eq!(earliest_second_to_mark_indices(&[1, 2, 3], &[1, 2, 3]), -1);
    }

    #[test]
    fn test() {
        assert_eq!(earliest_second_to_mark_indices(&[0], &[1]), 1);
    }
}
