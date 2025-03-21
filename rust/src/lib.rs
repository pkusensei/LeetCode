mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn min_cost(nums: &[i32], cost: &[i32]) -> i64 {
    let pairs = nums
        .iter()
        .copied()
        .zip(cost.iter().copied())
        .sorted()
        .collect_vec();
    let total: i64 = cost.iter().map(|&v| i64::from(v)).sum();
    let mut prefix = 0;
    let mut target = 0;
    for &(num, co) in pairs.iter() {
        prefix += i64::from(co);
        if prefix > total / 2 {
            target = num;
            break;
        }
    }
    pairs
        .into_iter()
        .map(|(num, co)| i64::from(co) * i64::from(num.abs_diff(target)))
        .sum()
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
        assert_eq!(min_cost(&[1, 3, 5, 2], &[2, 3, 1, 14]), 8);
        assert_eq!(min_cost(&[2, 2, 2, 2, 2], &[4, 2, 8, 1, 3]), 0);
    }

    #[test]
    fn test() {}
}
