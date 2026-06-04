mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut left = *weights.iter().max().unwrap_or(&1);
    let mut right = weights.iter().sum();
    while left < right {
        let mid = left + (right - left) / 2;
        if check(&weights, days, mid) {
            right = mid
        } else {
            left = 1 + mid
        }
    }
    left
}

fn check(weights: &[i32], mut days: i32, mid: i32) -> bool {
    let mut curr = 0;
    for &w in weights {
        if curr + w > mid {
            curr = w;
            days -= 1
        } else {
            curr += w;
        }
        if days <= 0 {
            return false;
        }
    }
    days >= 0
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
