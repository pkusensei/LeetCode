mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_largest(nums: &[i32], k: i32) -> i32 {
    let mut heap = std::collections::BinaryHeap::new();
    for &num in nums.iter() {
        heap.push(std::cmp::Reverse(num));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_sorted_vec().last().unwrap().0
    // *nums.select_nth_unstable(n - k as usize).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_kth_largest(&[3, 2, 1, 5, 6, 4], 2), 5);
        debug_assert_eq!(find_kth_largest(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn test() {}
}
