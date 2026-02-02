mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::{cmp::Reverse, collections::BinaryHeap};
type MinHeap = BinaryHeap<(Reverse<i64>, usize)>;
type MaxHeap = BinaryHeap<(i64, usize)>;

pub fn minimum_cost(nums: &[i32], k: i32, dist: i32) -> i64 {
    let n = nums.len();
    let [k, dist] = [k, dist].map(|v| v as usize);
    let mut res = i64::MAX >> 1;
    let mut max_heap = MaxHeap::new();
    let mut min_heap = MinHeap::new();
    let mut used = vec![false; n];
    let mut used_count = 0;
    let mut curr = 0_i64;
    for (right, &num) in nums.iter().enumerate().skip(1) {
        let num = i64::from(num);
        // Fix window size right-(1+left)=dist
        // left is one prior to window
        let left = right.checked_sub(1 + dist);
        if let Some(left) = left
            && left > 0
            && used[left]
        {
            curr = drop_left(
                nums,
                &mut max_heap,
                &mut min_heap,
                &mut used,
                &mut used_count,
                curr,
                left,
            );
        }
        if used_count < k - 1 {
            // Use right boundary
            curr = add_right(&mut max_heap, &mut used, &mut used_count, curr, right, num);
        } else {
            while let Some(top) = max_heap.peek()
                && !used[top.1]
            {
                max_heap.pop(); // pop unused
            }
            if let Some(&top) = max_heap.peek()
                && top.0 > num
            {
                // [right]<top(max_heap)
                curr = balance(&mut max_heap, &mut min_heap, &mut used, curr, right, num);
            } else {
                // Add to candidates
                min_heap.push((Reverse(num), right));
            }
        }
        if left.is_some() {
            res = res.min(curr);
        }
    }
    res + i64::from(nums[0])
}

fn balance(
    max_heap: &mut MaxHeap,
    min_heap: &mut MinHeap,
    used: &mut Vec<bool>,
    mut curr: i64,
    right: usize,
    num: i64,
) -> i64 {
    let top = max_heap.pop().unwrap();
    min_heap.push((Reverse(top.0), top.1));
    used[top.1] = false;
    max_heap.push((num, right));
    used[right] = true;
    curr += num - top.0;
    curr
}

fn add_right(
    max_heap: &mut MaxHeap,
    used: &mut [bool],
    used_count: &mut usize,
    curr: i64,
    right: usize,
    num: i64,
) -> i64 {
    used[right] = true;
    *used_count += 1;
    max_heap.push((num, right));
    curr + num
}

fn drop_left(
    nums: &[i32],
    max_heap: &mut MaxHeap,
    min_heap: &mut MinHeap,
    used: &mut [bool],
    used_count: &mut usize,
    mut curr: i64,
    left: usize,
) -> i64 {
    used[left] = false;
    *used_count -= 1;
    curr -= i64::from(nums[left]);
    while let Some((_, i)) = min_heap.peek()
        && *i < left
    {
        min_heap.pop();
    }
    // Find min unused num in window
    if let Some((Reverse(top), i)) = min_heap.pop() {
        used[i] = true;
        *used_count += 1;
        curr += top;
        max_heap.push((top, i));
    }
    curr
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
        assert_eq!(minimum_cost(&[1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(minimum_cost(&[10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(minimum_cost(&[10, 8, 18, 9], 3, 1), 36);
    }

    #[test]
    fn test() {}
}
