mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut nums: Vec<_> = grid.into_iter().flat_map(|r| r.into_iter()).collect();
    nums.sort_unstable();
    let n = nums.len();
    let med = nums[n / 2];
    let mut res = 0;
    for num in nums {
        let d = (med - num).abs();
        if d % x == 0 {
            res += d / x
        } else {
            return -1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
