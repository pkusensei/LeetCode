pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == 0 {
        return 0;
    }

    if let Some(r) = dividend.checked_div(divisor) {
        r
    } else if dividend.is_positive() == divisor.is_positive() {
        i32::MAX
    } else {
        i32::MIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(divide(10, 3), 3);
        debug_assert_eq!(divide(7, -3), -2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(divide(-2147483648, -1), i32::MAX)
    }
}
