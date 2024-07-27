pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut closest: i32 = nums[0..3].iter().sum();

    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let curr = nums[i] + nums[left] + nums[right];
            if (curr - target).abs() < (closest - target).abs() {
                closest = curr
            }
            match curr.cmp(&target) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Equal => return closest,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
    }

    closest
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        debug_assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        )
    }
}
