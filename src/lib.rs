mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(hamming_weight(11), 3);
        debug_assert_eq!(hamming_weight(128), 1);
        debug_assert_eq!(hamming_weight(2147483645), 30);
    }

    #[test]
    fn test() {}
}
