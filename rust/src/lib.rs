mod helper;

#[allow(unused_imports)]
use helper::*;

pub const fn add_digits(num: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Digital_root#Using_the_floor_function
    const BASE: i32 = 10;
    num - (BASE - 1) * ((num - 1) / (BASE - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(add_digits(38), 2);
        debug_assert_eq!(add_digits(0), 0);
    }

    #[test]
    fn test() {}
}
