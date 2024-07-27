pub fn max_area(height: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut res = 0;
    while right > left {
        let a = right - left;
        let h = if height[left] < height[right] {
            left += 1;
            height[left - 1]
        } else {
            right -= 1;
            height[right + 1]
        };
        res = res.max(a as i32 * h);
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        debug_assert_eq!(max_area(&[1, 1]), 1);
    }

    #[test]
    fn test() {}
}
