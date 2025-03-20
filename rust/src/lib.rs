mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn subarray_gcd(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for i1 in 0..n {
        let mut curr = 0;
        for i2 in i1..n {
            curr = gcd(curr, nums[i2]);
            if curr == k {
                res += 1;
            }
            if curr < k {
                break;
            }
        }
    }
    res
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
        assert_eq!(subarray_gcd(&[9, 3, 1, 2, 6, 3], 3), 4);
        assert_eq!(subarray_gcd(&[4], 7), 0);
    }

    #[test]
    fn test() {}
}
