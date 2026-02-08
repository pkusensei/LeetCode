mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::VecDeque;

pub fn count_subarrays(nums: &[i32], k: i64) -> i64 {
    let mut left = 0;
    let mut res = 0;
    let [mut minq, mut maxq] = [const { VecDeque::new() }; 2];
    for (right, &num) in nums.iter().enumerate() {
        while let Some((i, _val)) = minq.front()
            && *i < left
        {
            minq.pop_front();
        }
        while let Some(&(_, val)) = minq.back()
            && val > num
        {
            minq.pop_back();
        }
        minq.push_back((right, num));
        while let Some((i, _val)) = maxq.front()
            && *i < left
        {
            minq.pop_front();
        }
        while let Some(&(_, val)) = maxq.back()
            && val < num
        {
            maxq.pop_back();
        }
        maxq.push_back((right, num));
        while f(&minq, &maxq, right - left + 1) > k {
            if let Some((i, _)) = minq.front()
                && *i == left
            {
                minq.pop_front();
            }
            if let Some((i, _)) = maxq.front()
                && *i == left
            {
                maxq.pop_front();
            }
            left += 1;
        }
        let len = right - left + 1;
        res += len;
    }
    res as _
}

fn f(minq: &VecDeque<(usize, i32)>, maxq: &VecDeque<(usize, i32)>, len: usize) -> i64 {
    let (Some(a), Some(b)) = (minq.front(), maxq.front()) else {
        return 0;
    };
    i64::from(b.1 - a.1) * len as i64
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
        assert_eq!(count_subarrays(&[1, 3, 2], 4), 5);
        assert_eq!(count_subarrays(&[5; 4], 0), 10);
        assert_eq!(count_subarrays(&[1, 2, 3], 0), 3);
    }

    #[test]
    fn test() {}
}
