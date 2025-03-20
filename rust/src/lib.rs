mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right && nums[left] < nums[right] {
        match nums[left].abs().cmp(&nums[right]) {
            std::cmp::Ordering::Less => right -= 1,
            std::cmp::Ordering::Equal => return nums[right],
            std::cmp::Ordering::Greater => left += 1,
        }
    }
    -1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
