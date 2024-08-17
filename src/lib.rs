mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_right(nums: &mut [i32], k: i32) {
    let k = k as usize % nums.len();
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
            rotate_right(&mut nums, 3);
            debug_assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);
        }
        {
            let mut nums = vec![-1, -100, 3, 99];
            rotate_right(&mut nums, 2);
            debug_assert_eq!(nums, [3, 99, -1, -100]);
        }
    }

    #[test]
    fn test() {}
}
