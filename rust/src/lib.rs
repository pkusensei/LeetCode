mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter_map(|(k, v)| if v == 1 { Some(k) } else { None })
        .sum()
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
