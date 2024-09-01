mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n & (n - 1) == 0
    // n.count_ones() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_power_of_two(1));
        debug_assert!(is_power_of_two(16));
        debug_assert!(!is_power_of_two(3));
    }

    #[test]
    fn test() {
        debug_assert!(!is_power_of_two(0))
    }
}
