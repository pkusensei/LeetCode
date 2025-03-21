mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subarray_lcm(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for i1 in 0..n {
        let mut curr = nums[i1];
        for i2 in i1..n {
            curr = lcm(curr, nums[i2]);
            res += i32::from(curr == k);
            if curr > k {
                break;
            }
        }
    }
    res
}

const fn lcm(a: i32, b: i32) -> i32 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { b } else { gcd(b % a, a) }
    }
    a / gcd(a, b) * b
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
        assert_eq!(subarray_lcm(&[3, 6, 2, 7, 1], 6), 4);
        assert_eq!(subarray_lcm(&[3], 2), 0);
    }

    #[test]
    fn test() {}
}
