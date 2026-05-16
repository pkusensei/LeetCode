mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut res = 0;
    let mut queue = VecDeque::new();
    for (idx, &num) in nums.iter().enumerate() {
        while let Some(&prev_end) = queue.front()
            && prev_end < idx
        {
            queue.pop_front();
        }
        // num==1, even flips => no op
        // num==0, even flips => op
        if num == (queue.len() & 1) as i32 {
            if idx + k <= n {
                res += 1;
                queue.push_back(idx + k - 1);
            } else {
                return -1;
            }
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
