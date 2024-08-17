mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn rob(nums: &[i32]) -> i32 {
    // if nums.is_empty() {
    //     return 0;
    // }
    // if nums.len() == 1 {
    //     return nums[0];
    // }
    // let mut profit = Vec::with_capacity(nums.len());
    // profit.push(nums[0]);
    // profit.push(nums[0].max(nums[1]));
    // for (i, &num) in nums.iter().enumerate().skip(2) {
    //     profit.push(profit[i - 1].max(profit[i - 2] + num))
    // }
    // profit.into_iter().max().unwrap()

    let (mut res, mut prev) = (0, 0);
    for num in nums {
        let temp = res;
        res = res.max(prev + num);
        prev = temp
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(rob(&[1, 2, 3, 1]), 4);
        debug_assert_eq!(rob(&[2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test() {
        debug_assert_eq!(rob(&[2, 1]), 2);
        debug_assert_eq!(rob(&[2, 1, 1, 2]), 4);
    }
}
