mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    use itertools::Itertools;
    let map = nums.into_iter().counts();
    if map.len() < 3 {
        return 0;
    }
    map.values()
        .array_combinations::<3>()
        .map(|w| w.iter().copied().product::<usize>() as i32)
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
    fn basics() {}

    #[test]
    fn test() {}
}
