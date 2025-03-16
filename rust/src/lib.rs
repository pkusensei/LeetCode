mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &mut [i32], nums_divide: &[i32]) -> i32 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { b } else { gcd(b % a, a) }
    }

    let ngcd = nums_divide
        .iter()
        .fold(nums_divide[0], |acc, &v| gcd(acc, v));
    nums.sort_unstable();
    for (i, &num) in (0..).zip(nums.iter()) {
        if ngcd % num == 0 {
            return i;
        }
    }
    -1
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
        assert_eq!(min_operations(&mut [2, 3, 2, 4, 3], &[9, 6, 9, 3, 15]), 2);
    }

    #[test]
    fn test() {}
}
