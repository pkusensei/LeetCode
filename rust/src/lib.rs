mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// sum + (-2x)%k == prefix
// prefix + 2x%k == sum
pub fn longest_subarray(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let k = i64::from(k);
    let mut sum = 0;
    let mut first = vec![n as i32; k as usize];
    first[0] = -1;
    let mut last = vec![-1; k as usize];
    let mut double_pos = vec![vec![]; k as usize];
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        sum = (sum + num).rem_euclid(k);
        let i = sum as usize;
        // default to no negation
        res = res.max(idx as i32 - first[i]);
        first[i] = first[i].min(idx as i32);
        last[i] = idx as i32;
        double_pos[(2 * num).rem_euclid(k) as usize].push(idx as i32);
    }
    for (rem, &left) in first.iter().enumerate() {
        if left == n as i32 {
            continue;
        }
        for (d, pos) in double_pos.iter().enumerate() {
            let right = last[(rem + d) % k as usize];
            if right == -1 || pos.is_empty() {
                continue;
            }
            let i = pos.partition_point(|&v| v <= left);
            if pos.get(i).is_none_or(|&v| right < v) {
                continue;
            }
            res = res.max(right - left);
        }
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
        assert_eq!(longest_subarray(&[4, 1, 2], 3), 3);
    }

    #[test]
    fn test() {
        assert_eq!(longest_subarray(&[9, 13, 10], 5), 1);
    }
}
