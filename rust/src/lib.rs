mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::<_, Vec<_>>::new();
    for (i, &num) in nums.iter().enumerate() {
        map.entry(num).or_default().push(i);
    }
    map.values()
        .flat_map(|v| {
            v.windows(3)
                .map(|w| w[0].abs_diff(w[1]) + w[1].abs_diff(w[2]) + w[0].abs_diff(w[2]))
        })
        .min()
        .map(|v| v as i32)
        .unwrap_or(-1)
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
