mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_distance(nums: &[i32], s: &str, d: i32) -> i32 {
    let mut nums: Vec<_> = nums.iter().copied().map(i64::from).collect();
    for (v, dir) in nums.iter_mut().zip(s.bytes()) {
        if dir == b'R' {
            *v += i64::from(d);
        } else {
            *v -= i64::from(d);
        }
    }
    nums.sort_unstable();
    let mut prefix = 0;
    let mut res = 0;
    for (i, num) in (0..).zip(nums) {
        res += i * num - prefix;
        res %= 1_000_000_007;
        prefix += num;
    }
    res as i32
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
