pub fn search_range(nums: &[i32], target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }
    let left = nums.partition_point(|&n| n < target);

    if left == nums.len() || target != nums[left] {
        vec![-1, -1]
    } else {
        let mut curr = left;
        while nums.get(curr).is_some_and(|&n| n == target) {
            curr += 1;
        }
        vec![left as i32, curr as i32 - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(search_range(&[5, 7, 7, 8, 8, 10], 8), [3, 4]);
        debug_assert_eq!(search_range(&[5, 7, 7, 8, 8, 10], 6), [-1, -1]);
        debug_assert_eq!(search_range(&[], 0), [-1, -1]);
    }

    #[test]
    fn test() {}
}
