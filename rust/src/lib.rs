mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn missing_number(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    (1..=n)
        .chain(nums.iter().copied())
        .fold(0, |acc, num| acc ^ num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(missing_number(&[3, 0, 1]), 2);
        debug_assert_eq!(missing_number(&[0, 1]), 2);
        debug_assert_eq!(missing_number(&[9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn test() {}
}
