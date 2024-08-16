mod helper;

#[allow(unused_imports)]
use helper::*;

pub const fn trailing_zeroes(n: i32) -> i32 {
    if n < 5 {
        return 0;
    }
    let mut div = 5;
    let mut count = 0;
    while n / div > 0 {
        count += n / div;
        div *= 5;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(trailing_zeroes(3), 0);
        debug_assert_eq!(trailing_zeroes(5), 1);
        debug_assert_eq!(trailing_zeroes(0), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(trailing_zeroes(30), 7);
        debug_assert_eq!(trailing_zeroes(51), 12);
    }
}
