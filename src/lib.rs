pub fn jump(nums: &[i32]) -> i32 {
    let mut res = 0;
    let (mut slow, mut fast) = (0, 0);
    while fast < nums.len() - 1 {
        let long_jump = (slow..=fast)
            .map(|n| n + nums[n] as usize)
            .max()
            .unwrap_or_default();
        slow = fast + 1;
        fast = long_jump;
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(jump(&[2, 3, 1, 1, 4]), 2);
        debug_assert_eq!(jump(&[2, 3, 0, 1, 4]), 2)
    }

    #[test]
    fn test() {}
}
