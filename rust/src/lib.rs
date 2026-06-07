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

pub fn maximum_sum(nums: &[i32], m: i32, l: i32, r: i32) -> i64 {
    let [m, l, r] = [m, l, r].map(|v| v as usize);
    let n = nums.len();
    let prefix = nums.iter().fold(vec![0], |mut acc, &v| {
        acc.push(i64::from(v) + acc.last().unwrap_or(&0));
        acc
    });
    let mut prev = vec![0; 1 + n]; // zero subarr picked
    let mut res = i64::MIN >> 2;
    for _sub in 1..=m {
        let mut curr = vec![i64::MIN >> 2; 1 + n];
        let mut queue = VecDeque::<(usize, i64)>::new();
        let mut left = 0;
        for right in l..=n {
            // right-r..=right-l
            // prefix[right] + max(prev[left]-prefix[left])
            while left <= right - l {
                let num = prev[left] - prefix[left];
                while let Some(&(_, val)) = queue.back()
                    && val < num
                {
                    queue.pop_back();
                }
                while let Some(&(i, _)) = queue.front()
                    && i < right.saturating_sub(r)
                {
                    queue.pop_front();
                }
                queue.push_back((left, num));
                left += 1;
            }
            curr[right] =
                curr[right].max(queue.front().map(|&(_, v)| v).unwrap_or(0) + prefix[right]);
            res = res.max(curr[right]);
        }
        // Could be combined into dp directly
        // curr[right] = curr[right-1].max(...)
        for i in 1..=n {
            curr[i] = curr[i].max(curr[i - 1]);
        }
        prev = curr;
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
    fn basics() {
        assert_eq!(maximum_sum(&[4, 1, -5, 2], 2, 1, 3), 7);
        assert_eq!(maximum_sum(&[1, 0, 3, 4], 2, 1, 2), 8);
        assert_eq!(maximum_sum(&[-1, 7, -4], 1, 2, 3), 6);
        assert_eq!(maximum_sum(&[-3, -4, -1], 2, 1, 2), -1);
    }

    #[test]
    fn test() {}
}
