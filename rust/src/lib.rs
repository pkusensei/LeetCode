mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    use itertools::Itertools;
    let n = nums.len();
    let map = nums.iter().copied().counts();
    if k == 1 {
        map.into_iter()
            .filter_map(|(num, c)| if c == 1 { Some(num) } else { None })
            .max()
            .unwrap_or(-1)
    } else if k as usize == n {
        nums.into_iter().max().unwrap_or(-1)
    } else {
        match [map[&nums[0]], map[&nums[n - 1]]] {
            [1, 1] => nums[0].max(nums[n - 1]),
            [1, _] => nums[0],
            [_, 1] => nums[n - 1],
            _ => -1,
        }
    }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
