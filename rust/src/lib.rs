mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_arrivals_to_discard(arrivals: &[i32], w: i32, m: i32) -> i32 {
    use std::collections::{HashMap, HashSet};
    let mut left = 0;
    let mut freq = HashMap::new();
    let mut discard = HashSet::new();
    for (right, &num) in arrivals.iter().enumerate() {
        *freq.entry(num).or_insert(0) += 1;
        if right + 1 - left > w as usize {
            if !discard.contains(&left) {
                *freq.entry(arrivals[left]).or_insert(0) -= 1;
            }
            left += 1;
        }
        if freq[&num] > m {
            discard.insert(right);
            *freq.entry(num).or_insert(0) -= 1;
        }
    }
    discard.len() as i32
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
    fn basics() {
        assert_eq!(min_arrivals_to_discard(&[1, 2, 1, 3, 1], 4, 2), 0);
        assert_eq!(min_arrivals_to_discard(&[1, 2, 3, 3, 3, 4], 3, 2), 1);
    }

    #[test]
    fn test() {}
}
