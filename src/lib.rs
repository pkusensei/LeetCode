pub fn first_missing_positive(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut seen: Vec<_> = (0..=n).map(|_| false).collect();
    for &num in nums {
        if (1..=n).contains(&(num as usize)) {
            seen[num as usize] = true;
        }
    }
    seen.into_iter()
        .enumerate()
        .skip(1)
        .find_map(|(i, b)| if !b { Some(i as i32) } else { None })
        .unwrap_or(n as i32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(first_missing_positive(&[1, 2, 0]), 3);
        debug_assert_eq!(first_missing_positive(&[3, 4, -1, 1]), 2);
        debug_assert_eq!(first_missing_positive(&[7, 8, 9, 11, 12]), 1)
    }

    #[test]
    fn test() {}
}
