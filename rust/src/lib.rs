mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_smallest(coins: &[i32], k: i32) -> i64 {
    let k = i64::from(k);
    let [mut left, mut right] = [1, i64::MAX / 2];
    while left < right {
        let mid = left + (right - left) / 2;
        if count(&coins, mid) < k {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    left
}

fn count(nums: &[i32], mid: i64) -> i64 {
    let n = nums.len();
    let mut res = 0;
    // bitmask as subset
    for mask in 1_i32..(1 << n) {
        let mut curr_lcm = 1;
        for (i, &num) in nums.iter().enumerate() {
            if mask & (1 << i) > 0 {
                curr_lcm = lcm(curr_lcm, num.into());
            }
        }
        if mask.count_ones() & 1 == 0 {
            res -= mid / curr_lcm;
        } else {
            res += mid / curr_lcm;
        }
    }
    res
}

const fn lcm(a: i64, b: i64) -> i64 {
    const fn gcd(a: i64, b: i64) -> i64 {
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
        assert_eq!(find_kth_smallest(&[3, 6, 9], 3), 9);
        assert_eq!(find_kth_smallest(&[5, 2], 7), 12);
    }

    #[test]
    fn test() {}
}
