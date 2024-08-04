pub fn search(nums: &[i32], target: i32) -> bool {
    let pivot =
        nums.windows(2)
            .enumerate()
            .find_map(|(idx, w)| if w[0] > w[1] { Some(idx + 1) } else { None });
    if let Some(pivot) = pivot {
        binary_search(&nums[0..pivot], target) || binary_search(&nums[pivot..], target)
    } else {
        binary_search(nums, target)
    }
}

fn binary_search(nums: &[i32], target: i32) -> bool {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = (right - left) / 2 + left;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater if left == right => {
                return false
            }
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(search(&[2, 5, 6, 0, 0, 1, 2], 0));
        debug_assert!(!search(&[2, 5, 6, 0, 0, 1, 2], 3));
        debug_assert!(search(&[1, 0, 1, 1, 1], 0));
    }

    #[test]
    fn test() {
        debug_assert!(search(&[2, 2, 2, 3, 2, 2, 2], 3))
    }
}
