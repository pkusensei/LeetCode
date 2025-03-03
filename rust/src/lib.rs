mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut res = 0;
        while num1 != 0 && num2 != 0 {
            if num1 >= num2 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            res += 1;
        }
        res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_time("1100101"), 5);
        assert_eq!(minimum_time("0010"), 2);
    }

    #[test]
    fn test() {}
}
