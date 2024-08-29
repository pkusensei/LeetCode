mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn contains_nearby_almost_duplicate(nums: &[i32], index_diff: i32, value_diff: i32) -> bool {
    let size = nums.len();
    if size == 0 || index_diff < 0 || value_diff < 0 {
        return false;
    }
    // A combination of sliding window and bucket sort
    // bucket size: 1+value_diff => [0..value_diff]
    // window size: 1+index_diff => [idx..idx+index_diff]
    let mut buckets = HashMap::new();
    for (idx, &num) in nums.iter().enumerate() {
        let bucket = get_bucket(num, value_diff);
        if buckets.insert(bucket, num).is_some() {
            // insert one number to make window size 1+index_diff
            return true;
        }
        if buckets
            .get(&(bucket - 1))
            .is_some_and(|v| num - v <= value_diff)
        {
            // check left bucket
            return true;
        }
        if buckets
            .get(&(bucket + 1))
            .is_some_and(|v| v - num <= value_diff)
        {
            // check right bucket
            return true;
        }
        // Failing all above ifs means all buckets contain only single number
        // And neighboring buckets don't have numbers that fit the criteria
        if buckets.len() > index_diff as usize {
            // buckets is a sliding window of size 1+index_diff due to insert above
            // to prepare next loop, remove oldest inserted number
            let value = nums[idx - index_diff as usize];
            let key = get_bucket(value, value_diff);
            buckets.remove(&key);
        }
    }
    false
}

fn get_bucket(num: i32, value_diff: i32) -> i32 {
    let mut bucket = num / (value_diff + 1);
    if num < 0 {
        bucket -= 1;
    }
    bucket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(contains_nearby_almost_duplicate(&[1, 2, 3, 1], 3, 0));
        debug_assert!(!contains_nearby_almost_duplicate(&[1, 5, 9, 1, 5, 9], 2, 3));
    }

    #[test]
    fn test() {
        debug_assert!(contains_nearby_almost_duplicate(&[-3, 3, -6], 2, 3))
    }
}
