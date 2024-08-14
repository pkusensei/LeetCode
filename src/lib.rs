mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_product(nums: &[i32]) -> i32 {
    // NOTE: cast to f64 for extreme test cases
    if nums.len() < 2 {
        return nums.first().copied().unwrap_or(0);
    }
    let (mut curr_min, mut curr_max) = (nums[0], nums[0]);
    let mut res = curr_max;
    for &num in &nums[1..] {
        if num == 0 {
            (curr_min, curr_max) = (1, 1);
            res = res.max(0)
        } else {
            let temp = num.max(curr_max * num).max(curr_min * num);
            curr_min = num.min(curr_max * num).min(curr_min * num);
            curr_max = temp;
            res = res.max(curr_max)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_product(&[2, 3, -2, 4]), 6);
        debug_assert_eq!(max_product(&[-2, 0, -1]), 0);
    }

    #[test]
    fn test() {}
}
