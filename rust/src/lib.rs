mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// 01
// 10
// 11
pub fn min_removals(nums: &[i32], target: i32) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut half = vec![0];
    let full_mask: usize = 1 << (n / 2);
    for mask in 1..full_mask {
        let i = mask & mask.wrapping_neg();
        half.push(half[mask ^ i] ^ nums[i.ilog2() as usize]);
    }
    let mut half_map = HashMap::new();
    for (mask, &xor) in half.iter().enumerate() {
        let v = half_map.entry(xor).or_insert(mask.count_ones());
        *v = (*v).max(mask.count_ones());
    }
    half = vec![0];
    let full_mask: usize = 1 << (n - n / 2);
    let mut res = half_map.get(&target).copied();
    for mask in 1..full_mask {
        let i = mask & mask.wrapping_neg();
        let curr = half[mask ^ i] ^ nums[i.ilog2() as usize + n / 2];
        half.push(curr);
        if let Some(v) = half_map.get(&(target ^ curr)) {
            res = res.max(Some(v + mask.count_ones()));
        }
    }
    if let Some(v) = res {
        n as i32 - v as i32
    } else if target == 0 {
        n as i32
    } else {
        -1
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
    fn basics() {
        assert_eq!(min_removals(&[1, 2, 3], 2), 1);
    }

    #[test]
    fn test() {
        assert_eq!(min_removals(&[7, 6], 0), 2);
    }
}
