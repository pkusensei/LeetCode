mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut res = i32::MAX;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let rev = {
            let mut v = 0;
            let mut x = num;
            while x > 0 {
                v = 10 * v + x % 10;
                x /= 10;
            }
            v
        };
        if let Some(v) = map.get(&num) {
            res = res.min(i as i32 - v)
        }
        *map.entry(rev).or_insert(i as i32) = i as i32;
    }
    if res == i32::MAX { -1 } else { res }
}

#[allow(unused_imports)]
use helper::*;

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
