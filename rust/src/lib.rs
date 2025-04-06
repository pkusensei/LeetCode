mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
    let idx: Vec<_> = nums
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 1 { Some(i) } else { None })
        .collect();
    if idx.is_empty() {
        return 0;
    }
    idx.windows(2)
        .fold(1, |acc, w| acc * (w[1] - w[0]) % 1_000_000_007) as i32
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
