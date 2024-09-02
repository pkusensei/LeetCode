mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_ugly(mut n: i32) -> bool {
    for x in [2, 3, 5] {
        while n > 1 && n % x == 0 {
            n /= x
        }
    }
    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_ugly(6));
        debug_assert!(is_ugly(1));
        debug_assert!(!is_ugly(14));
    }

    #[test]
    fn test() {}
}
