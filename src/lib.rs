use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums = HashSet::<i32>::from_iter(nums);
    let mut res = 0;

    for &num in nums.iter() {
        if !nums.contains(&(num - 1)) {
            let mut curr = num;
            let mut streak = 1;
            while nums.contains(&(curr + 1)) {
                curr += 1;
                streak += 1;
            }
            res = res.max(streak);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        debug_assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }

    #[test]
    fn test() {}
}
