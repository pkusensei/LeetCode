mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    (low_limit..=high_limit)
        .fold(std::collections::HashMap::new(), |mut acc, mut num| {
            let mut curr = 0;
            while num > 0 {
                curr += num % 10;
                num /= 10;
            }
            *acc.entry(curr).or_insert(0) += 1;
            acc
        })
        .into_values()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
