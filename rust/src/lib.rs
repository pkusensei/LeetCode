mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn find_closest_elements(arr: &[i32], k: i32, x: i32) -> Vec<i32> {
    let n = arr.len();
    let k = k as usize;
    let pivot = arr.partition_point(|&n| n < x);
    if pivot == 0 {
        arr.iter().copied().take(k).collect()
    } else if pivot == n {
        arr.iter().rev().copied().take(k).rev().collect()
    } else {
        let (mut left, mut right) = (pivot - 1, pivot);
        let mut heap = BinaryHeap::new();
        heap.push((Reverse((arr[left].abs_diff(x), arr[left])), left));
        heap.push((Reverse((arr[right].abs_diff(x), arr[right])), right));
        let mut res = Vec::with_capacity(k);
        while res.len() < k {
            let Some((Reverse((_, num)), idx)) = heap.pop() else {
                break;
            };
            if idx < pivot {
                res.insert(0, num);
                if idx > 0 {
                    left = idx - 1;
                    heap.push((Reverse((arr[left].abs_diff(x), arr[left])), left));
                }
            } else {
                res.push(num);
                if idx < n - 1 {
                    right = idx + 1;
                    heap.push((Reverse((arr[right].abs_diff(x), arr[right])), right));
                }
            }
        }
        res
    }
}

fn with_binary_search(arr: &[i32], k: usize, x: i32) -> &[i32] {
    let (mut left, mut right) = (0, arr.len() - k);
    while left < right {
        let mid = left + (right - left) / 2;
        if x.abs_diff(arr[mid]) <= x.abs_diff(arr[mid + k]) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    &arr[left..(left + k)]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_binary_search(&[1, 2, 3, 4, 5], 4, 3), [1, 2, 3, 4]);
        debug_assert_eq!(with_binary_search(&[1, 1, 2, 3, 4, 5], 4, -1), [1, 1, 2, 3]);
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
