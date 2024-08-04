pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as _;
    }
    let (mut slow, mut fast) = (0, 2);
    while fast < nums.len() {
        if nums[slow] == nums[fast] {
            nums.remove(fast);
        } else {
            slow += 1;
            fast += 1
        }
    }
    nums.len() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
        debug_assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
    }

    #[test]
    fn test() {}
}
