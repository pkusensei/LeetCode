mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_moves(nums: &[i32], k: i32, max_changes: i32) -> i64 {
    let prefix = nums.iter().enumerate().fold(vec![0], |mut acc, (i, &v)| {
        if v > 0 {
            acc.push(i as i64 + acc.last().unwrap_or(&0));
        }
        acc
    });
    let n = prefix.len() - 1;
    // the least number of 1's picked with swap
    let need = (k - max_changes).max(0) as usize;
    let mut res = i64::MAX;
    for left in need..=(k as usize).min(3 + need).min(n) {
        for i in 0..=n - left {
            let mid1 = i + left / 2;
            let mid2 = i + left - left / 2;
            // sum of right cost + sum of left cost
            // On right, cost = i-mid
            // On left, cost = mid-i
            // right_indices - mid_indices + mid_indices - left_indices
            let curr = prefix[i + left] - prefix[mid2] - (prefix[mid1] - prefix[i]);
            res = res.min(curr + (i64::from(k) - left as i64) * 2);
        }
    }
    res
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
        assert_eq!(minimum_moves(&[1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1), 3);
        assert_eq!(minimum_moves(&[0, 0, 0, 0], 2, 3), 4);
    }

    #[test]
    fn test() {}
}
