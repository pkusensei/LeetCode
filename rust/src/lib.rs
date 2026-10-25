mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

// (1+316)*316/2 = 50086
pub fn count_staircase(nums: &[i32]) -> i32 {
    let n = nums.len();
    // num -> [indices]
    let mut occurs = HashMap::<_, Vec<_>>::new();
    // indice -> pos in array of same num
    let mut pos_in_occ = Vec::with_capacity(n);
    for (idx, &num) in nums.iter().enumerate() {
        let v = occurs.entry(num).or_default();
        pos_in_occ.push(v.len());
        v.push(idx);
    }
    (0..n).map(|i| f(&nums, i, &occurs, &pos_in_occ)).sum()
}

fn f(nums: &[i32], start: usize, occurs: &HashMap<i32, Vec<usize>>, pos_in_occ: &[usize]) -> i32 {
    let n = nums.len();
    let mut seen = HashSet::new();
    // Smallest right end of subarr
    let mut range_left = 0;
    // Biggest right end
    let mut range_right = n - 1;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate().skip(start) {
        if right > range_right {
            break;
        }
        if !seen.insert(num) {
            continue; // care only first timers
        }
        let k = seen.len();
        let occur = &occurs[&num];
        if pos_in_occ[right] + k - 1 >= occur.len() {
            break; // too few
        }
        let new_left = occur[pos_in_occ[right] + k - 1];
        let new_right = if pos_in_occ[right] + k < occur.len() {
            occur[pos_in_occ[right] + k]
        } else {
            n - 1 // depleted; default to end of array
        };
        range_left = range_left.max(new_left);
        range_right = range_right.min(new_right);
        if range_left > range_right {
            break;
        };
        let len = k * (1 + k) / 2;
        let last = start + len - 1;
        res += i32::from((range_left..=range_right).contains(&last));
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(count_staircase(&[5, 8, 8, 2, 2, 2]), 9);
    }

    #[test]
    fn test() {}
}
