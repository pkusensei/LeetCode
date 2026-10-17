mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    use itertools::Itertools;
    let n = nums.len();
    let indices = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let mut x = v;
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            (sum, v, i)
        })
        .sorted_unstable()
        .map(|(_, _, i)| i)
        .collect_vec();
    let mut target = vec![0; n];
    for (target_idx, &nums_idx) in indices.iter().enumerate() {
        target[nums_idx] = target_idx;
    }
    let mut seen = vec![false; n];
    let mut cycles = 0;
    for idx in 0..n {
        if !seen[idx] {
            cycles += 1;
            let mut i = idx;
            while !seen[i] {
                seen[i] = true;
                i = target[i];
            }
        }
    }
    (n - cycles) as i32
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
    fn basics() {}

    #[test]
    fn test() {}
}
