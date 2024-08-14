mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_min(nums: &[i32]) -> i32 {
    if nums.len() < 2 {
        return nums[0];
    }
    let (mut low, mut high) = (0, nums.len() - 1);
    while low < high {
        let mid = (high - low) / 2 + low;
        match nums[mid].cmp(&nums[high]) {
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Equal => (),
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
        debug_assert_eq!(find_min(&[3, 4, 5, 1, 2]), 1);
        debug_assert_eq!(find_min(&[4, 5, 6, 7, 0, 1, 2]), 0);
        debug_assert_eq!(find_min(&[11, 13, 15, 17]), 11);
    }

    #[test]
    fn test() {}
}
