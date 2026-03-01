mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_subarrays(nums: &[i32], k: i32, m: i32) -> i64 {
    use std::collections::HashMap;
    let mut res = 0;
    let [mut unique_map, mut valid_map] = [HashMap::new(), HashMap::new()];
    let [mut u_left, mut v_left] = [0, 0];
    let mut valid = 0;
    for (_right, &num) in nums.iter().enumerate() {
        // Window [u_left..=right] has k elements
        *unique_map.entry(num).or_insert(0) += 1;
        while unique_map.len() > k as usize {
            let f = unique_map.entry(nums[u_left]).or_insert(0);
            *f -= 1;
            if *f == 0 {
                unique_map.remove(&nums[u_left]);
            }
            u_left += 1;
        }

        let f = valid_map.entry(num).or_insert(0);
        *f += 1;
        if *f == m {
            valid += 1; // cross the valid boundary
        }
        // Window [v_left-1 ..=right] has k "valid" numbers
        while valid >= k {
            let f = valid_map.entry(nums[v_left]).or_insert(0);
            if *f == m {
                valid -= 1; // cross the valid boundary
            }
            *f -= 1;
            if *f == 0 {
                valid_map.remove(&nums[v_left]);
            }
            v_left += 1;
        }
        // [u_left..vleft] are left boundaries for valid window
        res += v_left.saturating_sub(u_left);
    }
    res as i64
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
        assert_eq!(count_subarrays(&[1, 2, 1, 2, 2], 2, 2), 2);
        assert_eq!(count_subarrays(&[3, 1, 2, 4], 2, 1), 3);
    }

    #[test]
    fn test() {}
}
