mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    use itertools::Itertools;
    let map = nums.iter().map(|v| v.rem_euclid(value)).counts();
    (0..value)
        .map(|val| *map.get(&val).unwrap_or(&0) as i32 * value + val)
        .min()
        .unwrap()
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
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(
            find_smallest_integer(vec![3, 0, 3, 2, 4, 2, 1, 1, 0, 4], 5),
            10
        );
    }
}
