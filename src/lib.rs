pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let ones = nums.iter().sum::<i32>() as _;
    if !(2..nums.len() - 1).contains(&ones) {
        return 0;
    }
    let mut expanded = nums.clone();
    expanded.extend(nums);
    let mut prev = 0;
    let mut res = i32::MAX;
    for i in 0..expanded.len() / 2 {
        prev = count(&expanded, i, ones, prev);
        res = res.min(prev);
    }
    res
}

fn count(nums: &[i32], start: usize, size: usize, prev: i32) -> i32 {
    if start == 0 {
        nums[0..size].iter().filter(|&&n| n == 0).count() as _
    } else {
        let front = if nums[start - 1] == 0 { -1 } else { 0 };
        let back = if nums[start + size - 1] == 0 { 1 } else { 0 };
        prev + front + back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
        debug_assert_eq!(min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
        debug_assert_eq!(min_swaps(vec![1, 1, 0, 0, 1]), 0);
    }

    #[test]
    fn test() {}
}
