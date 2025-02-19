mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_the_difference(mat: &[&[i32]], target: i32) -> i32 {
    let min_sum: i32 = mat.iter().map(|r| r.iter().min().unwrap_or(&0)).sum();
    if min_sum >= target {
        return min_sum - target;
    }
    let mut set = HashSet::from([0]);
    for row in mat.iter() {
        let mut next = HashSet::new();
        for &prev in set.iter() {
            for &num in row.iter() {
                if prev + num <= 2 * target - min_sum {
                    next.insert(prev + num);
                }
            }
        }
        set = next;
    }
    set.into_iter()
        .map(|v| (v - target).abs())
        .min()
        .unwrap_or_default()
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
        assert_eq!(
            minimize_the_difference(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]], 13),
            0
        );
        assert_eq!(minimize_the_difference(&[&[1], &[2], &[3]], 100), 94);
        assert_eq!(minimize_the_difference(&[&[1, 2, 9, 8, 7]], 6), 1);
    }

    #[test]
    fn test() {}
}
