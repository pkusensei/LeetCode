mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn single_number(nums: &[i32]) -> Vec<i32> {
    let mask = nums.iter().skip(1).fold(nums[0], |acc, &num| acc ^ num);
    let mut i = 0;
    while (mask >> i) & 1 == 0 {
        i += 1;
    }
    let bit = 1 << i;
    let a = nums
        .iter()
        .filter(|&num| num & bit != 0)
        .fold(0, |acc, num| acc ^ num);
    // let b = nums
    //     .iter()
    //     .filter(|&num| num & bit == 0)
    //     .fold(0, |acc, num| acc ^ num);
    vec![a, mask ^ a]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(single_number(&[1, 2, 1, 3, 2, 5]), [3, 5]);
        debug_assert_eq!(single_number(&[-1, 0]), [-1, 0]);
        debug_assert_eq!(single_number(&[0, 1]), [1, 0]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(single_number(&[1, 1, 0, -2147483648]), [-2147483648, 0])
    }
}
