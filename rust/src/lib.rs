mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
    let mut sum_xor = 0;
    let mut min_inc = i32::MAX;
    let mut count = 0;
    let mut sum = 0;
    let mut min_dec = i32::MAX;
    for &num in nums.iter() {
        let xor = num ^ k;
        if xor > num {
            sum_xor += i64::from(xor);
            count += 1;
            min_inc = min_inc.min(xor - num);
        } else {
            sum += i64::from(num);
            min_dec = min_dec.min(num - xor);
        }
    }
    if count & 1 == 0 {
        sum + sum_xor
    } else {
        (sum_xor - i64::from(min_inc) + sum).max(sum_xor + sum - i64::from(min_dec))
    }
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
