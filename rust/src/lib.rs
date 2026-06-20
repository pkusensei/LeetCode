mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn heap_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in (0..=(n / 2 - 1)).rev() {
        heapify(nums, n, i);
    }
    for i in (0..n).rev() {
        nums.swap(0, i);
        heapify(nums, i, 0);
    }
}

/// Build max heap
fn heapify(nums: &mut [i32], n: usize, idx: usize) {
    // Assume `idx` is largest
    let mut largest = idx;
    let left = 2 * idx + 1;
    let right = 2 * idx + 2;
    if left < n && nums[left] > nums[largest] {
        largest = left;
    }
    if right < n && nums[right] > nums[largest] {
        largest = right;
    }
    // But `idx` has bigger child node(s)
    if largest != idx {
        nums.swap(idx, largest);
        // Try the new `largest`
        heapify(nums, n, largest);
    }
}

pub fn radix_sort(nums: &mut [i32]) {
    let mut max = *nums.iter().max().unwrap_or(&0);
    let mut width = 0;
    while max > 0 {
        width += 1;
        max /= 10;
    }
    let mut place_value = 1;
    for _ in 0..width {
        bucket_sort(nums, place_value);
        place_value *= 10;
    }
}

fn bucket_sort(nums: &mut [i32], place_value: i32) {
    let mut buckets = [const { vec![] }; 10];
    for &num in nums.iter() {
        let d = num.abs() / place_value;
        buckets[(d % 10) as usize].push(num);
    }
    for (slot, val) in nums.iter_mut().zip(buckets.into_iter().flatten()) {
        *slot = val;
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
        {
            let mut v = [3, 2, 4, 5, 1];
            heap_sort(&mut v);
            assert_eq!(v, [1, 2, 3, 4, 5])
        }
        {
            let mut v = [3, 2, 4, 5, 1];
            radix_sort(&mut v);
            assert_eq!(v, [1, 2, 3, 4, 5])
        }
    }

    #[test]
    fn test() {}
}
