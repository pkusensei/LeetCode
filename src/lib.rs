mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn rob_2(nums: &[i32]) -> i32 {
    let size = nums.len();
    if size <= 2 {
        return nums.first().max(nums.last()).copied().unwrap_or(0);
    }
    rob_1(&nums[1..]).max(rob_1(&nums[..size - 1]))
}

fn rob_1(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut dp = Vec::with_capacity(nums.len());
    dp.push(nums[0]);
    dp.push(nums[0].max(nums[1]));
    for (i, &num) in nums.iter().enumerate().skip(2) {
        dp.push(dp[i - 1].max(dp[i - 2] + num))
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(rob_2(&[2, 3, 2]), 3);
        debug_assert_eq!(rob_2(&[1, 2, 3, 1]), 4);
        debug_assert_eq!(rob_2(&[1, 2, 3]), 3);
    }

    #[test]
    fn test() {}
}
