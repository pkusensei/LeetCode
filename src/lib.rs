mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    fn bit_shift(mut left: i32, mut right: i32) -> i32 {
        let mut shift = 0;
        while left < right {
            left >>= 1;
            right >>= 1;
            shift += 1;
        }
        left << shift
    }

    if left == 0 {
        return 0;
    }
    let (left_p, right_p) = (left.ilog2(), right.ilog2());
    if left_p == right_p {
        (left..=right).fold(left, |acc, n| acc & n)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(range_bitwise_and(5, 7), 4);
        debug_assert_eq!(range_bitwise_and(0, 0), 0);
        debug_assert_eq!(range_bitwise_and(1, i32::MAX), 0);
    }

    #[test]
    fn test() {}
}
