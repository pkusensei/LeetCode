pub fn sort_colors(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    let (mut left, mut right) = (0, nums.len() - 1);
    let mut i = 0;
    while left < right && i <= right {
        if nums[i] == 0 {
            nums.swap(i, left);
            left += 1;
            i += 1;
        } else if nums[i] == 2 {
            nums.swap(i, right);
            right -= 1;
        } else {
            i += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut v = [2, 0, 2, 1, 1, 0];
            sort_colors(&mut v);
            debug_assert_eq!(v, [0, 0, 1, 1, 2, 2]);
        }
        {
            let mut v = [2, 0, 1];
            sort_colors(&mut v);
            debug_assert_eq!(v, [0, 1, 2]);
        }
    }

    #[test]
    fn test() {}
}
