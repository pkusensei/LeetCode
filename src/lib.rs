pub fn can_jump(nums: &[i32]) -> bool {
    let (mut slow, mut fast) = (0, 0);
    while fast < nums.len() - 1 {
        let long_jump = (slow..=fast)
            .map(|n| n + nums[n] as usize)
            .max()
            .unwrap_or_default();
        slow = fast + 1;
        fast = fast.max(long_jump);
        if fast < slow {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_jump(&[2, 3, 1, 1, 4]));
        debug_assert!(!can_jump(&[3, 2, 1, 0, 4]));
    }

    #[test]
    fn test() {
        debug_assert!(can_jump(&[2, 5, 0, 0]));
        debug_assert!(can_jump(&[1, 2]));
        debug_assert!(can_jump(&[5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]));
    }
}
