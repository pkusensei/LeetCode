mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_peak_element(nums: &[i32]) -> i32 {
    let (mut low, mut high) = (0, nums.len() - 1);
    while low < high {
        let mid = (high - low) / 2 + low;
        match nums[mid].cmp(&nums[mid + 1]) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    low as i32

    // -inf, [low..mid..high], -inf
    // [mid] < [mid+1] is akin to -inf < [low] => search [mid+1..high]
    // [mid] > [mid+1] is akin to [high] > -inf => search [low..mid]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_peak_element(&[1, 2, 3, 1]), 2);
        debug_assert_eq!(find_peak_element(&[1, 2, 1, 3, 5, 6, 4]), 5); // or 1
    }

    #[test]
    fn test() {}
}
