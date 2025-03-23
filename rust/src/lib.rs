mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_total_cost(nums1: &[i32], nums2: &[i32]) -> i64 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut res = 0;
    let mut equals = 0;
    for (i, (a, b)) in nums1.iter().zip(nums2.iter()).enumerate() {
        if a == b {
            res += i as i64;
            equals += 1;
            *map.entry(*a).or_insert(0) += 1;
        }
    }
    let Some((max_num, max_count)) = map.into_iter().max_by_key(|&(_k, v)| v) else {
        return 0;
    };
    // (equal - max_count) => swaps within equal elements
    // swaps = max_count - above => necessary swaps with unequal elements
    let mut swaps = max_count - (equals - max_count);
    for (i, (&a, &b)) in nums1.iter().zip(nums2.iter()).enumerate() {
        if swaps <= 0 {
            break;
        }
        if a != b && a != max_num && b != max_num {
            res += i as i64;
            swaps -= 1;
        }
    }
    if swaps > 0 { -1 } else { res }
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
        assert_eq!(minimum_total_cost(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5],), 10);
        assert_eq!(minimum_total_cost(&[2, 2, 2, 1, 3], &[1, 2, 2, 3, 3]), 10);
        assert_eq!(minimum_total_cost(&[1, 2, 2], &[1, 2, 2]), -1);
    }

    #[test]
    fn test() {}
}
