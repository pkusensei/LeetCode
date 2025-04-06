mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    use itertools::Itertools;
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { b } else { gcd(b % a, a) }
    }

    let ds = nums
        .iter()
        .map(|num| {
            let mut num = *num;
            let last = num % 10;
            let mut first = 0;
            while num > 0 {
                first = num % 10;
                num /= 10;
            }
            [first, last]
        })
        .collect_vec();
    let mut res = 0;
    for (i, &[a, _]) in ds.iter().enumerate() {
        for &[_, b] in ds.iter().skip(1 + i) {
            res += i32::from(gcd(a, b) == 1);
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
