mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_min(mut nums: Vec<i32>) -> i32 {
    while nums.len() > 1 && nums.first() == nums.last() {
        nums.pop();
    }
    let (mut low, mut high) = (0, nums.len() - 1);
    while low < high {
        let mid = (high - low) / 2 + low;
        match nums[mid].cmp(&nums[high]) {
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Equal => high -= 1,
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }
    nums[low]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_min(vec![1, 3, 5]), 1);
        debug_assert_eq!(find_min(vec![2, 2, 2, 0, 1]), 0);
        debug_assert_eq!(find_min(vec![1, 1]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_min(vec![1, 1, 3]), 1)
    }
}
