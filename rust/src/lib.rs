mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut mask = 0;
    for bit in (0..31).rev() {
        mask |= 1 << bit;
        let mut count = k;
        let mut and = mask;
        for &num in &nums {
            and &= num;
            if and | res == res {
                and = mask;
            } else {
                count -= 1;
            }
        }
        if count < 0 {
            res |= 1 << bit;
        }
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
        assert_eq!(min_or_after_operations(vec![3, 5, 3, 2, 7], 2), 3);
        assert_eq!(min_or_after_operations(vec![7, 3, 15, 14, 2, 8], 4), 2);
        assert_eq!(
            min_or_after_operations(vec![10, 7, 10, 3, 9, 14, 9, 4], 1),
            15
        );
    }

    #[test]
    fn test() {
        assert_eq!(min_or_after_operations(vec![39, 62, 35, 11, 28, 32], 3), 38);
    }
}
