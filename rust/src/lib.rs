mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn contains_nearby_duplicate(nums: &[i32], k: i32) -> bool {
    let k = k as usize;
    let mut map = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        if let Some(i) = map.get(num) {
            if idx - *i <= k {
                return true;
            }
        }
        map.insert(num, idx);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(contains_nearby_duplicate(&[1, 2, 3, 1], 3));
        debug_assert!(contains_nearby_duplicate(&[1, 0, 1, 1], 1));
        debug_assert!(!contains_nearby_duplicate(&[1, 2, 3, 1, 2, 3], 2));
    }

    #[test]
    fn test() {
        debug_assert!(contains_nearby_duplicate(&[99, 99], 2))
    }
}
