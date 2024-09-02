mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let (mut prefix, mut suffix) = (vec![1; n], vec![1; n + 1]);
    for (idx, &num) in nums.iter().enumerate().take(n - 1) {
        prefix[idx + 1] = num * prefix[idx];
    }
    for (idx, &num) in nums.iter().enumerate().rev().take(n - 1) {
        suffix[idx - 1] = num * suffix[idx];
    }
    (0..n).map(|i| prefix[i] * suffix[i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(product_except_self(&[1, 2, 3, 4]), [24, 12, 8, 6]);
        debug_assert_eq!(product_except_self(&[-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
    }

    #[test]
    fn test() {}
}
