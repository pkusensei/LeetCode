mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_nice_pairs(nums: &[i32]) -> i32 {
    nums.iter()
        .fold(std::collections::HashMap::new(), |mut acc, &num| {
            *acc.entry(num - reverse(num)).or_insert(0i64) += 1;
            acc
        })
        .into_values()
        .fold(0, |acc, v| (acc + v * (v - 1) / 2) % 1_000_000_007) as _
}

const fn reverse(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        res = 10 * res + num % 10;
        num /= 10;
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
