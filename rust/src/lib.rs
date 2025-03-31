mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32]) -> i32 {
    let n = nums.len();
    let ones = nums.iter().filter(|&&v| v == 1).count();
    if ones > 0 {
        return (n - ones) as i32;
    }
    let mut res = n;
    for (left, &a) in nums.iter().enumerate() {
        let mut val = a;
        for (right, &b) in nums[left..].iter().enumerate() {
            val = gcd(val, b);
            if val == 1 {
                res = res.min(right);
                break;
            }
        }
    }
    if res == n { -1 } else { (res + n - 1) as i32 }
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(min_operations(&[2, 6, 3, 4]), 4);
        assert_eq!(min_operations(&[2, 10, 6, 14]), -1);
    }

    #[test]
    fn test() {
        assert_eq!(min_operations(&[1, 1]), 0);
    }
}
