mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    nums.iter()
        .enumerate()
        .filter_map(|(idx, &num)| {
            if idx.checked_sub(k).is_none_or(|i| nums[i] < num)
                && nums.get(idx + k).is_none_or(|&v| v < num)
            {
                Some(num)
            } else {
                None
            }
        })
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
