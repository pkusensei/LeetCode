pub fn trap(height: &[i32]) -> i32 {
    if height.len() < 2 {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];
    let mut res = 0;
    while left < right {
        if left_max < right_max {
            left += 1;
            left_max = left_max.max(height[left]);
            res += left_max - height[left];
        } else {
            right -= 1;
            right_max = right_max.max(height[right]);
            res += right_max - height[right];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        debug_assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test() {}
}
