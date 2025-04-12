mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
    let Some(start) = nums.windows(2).position(|w| w[0] > w[1]) else {
        return 0;
    };
    let n = nums.len();
    let mut i = (1 + start) % n;
    while i != start {
        if nums[i] > nums[(i + 1) % n] {
            return -1;
        }
        i = (1 + i) % n;
    }
    (n - start - 1) as i32
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
