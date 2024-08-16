mod helper;

#[allow(unused_imports)]
use helper::*;

// Boyer–Moore majority vote algorithm
pub fn majority_element(nums: &[i32]) -> i32 {
    nums.iter()
        .fold((0, 0), |(count, res), &num| {
            let temp = if count == 0 { num } else { res };
            let count = if temp == num { count + 1 } else { count - 1 };
            (count, temp)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(majority_element(&[3, 2, 3]), 3);
        debug_assert_eq!(majority_element(&[2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test() {}
}
