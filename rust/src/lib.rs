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
    let [l, r] = [l, r].map(|v| v as usize);
    let prefix = nums.iter().fold(vec![0], |mut acc, &v| {
        acc.push(i64::from(v) + acc.last().unwrap_or(&0));
        acc
    });
    let max_single = single(&prefix, l, r);
    let (free_val, free_count) = solve(&prefix, l, r, 0);
    if free_val <= 0 {
        return max_single;
    }
    if free_count <= m {
        return free_val;
    }
    let mut low = 0;
    let mut high = 1 + max_single;
    while low < high {
        let mid = low + (high - low + 1) / 2;
        let (_, count) = solve(&prefix, l, r, mid);
        if count >= m {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    let (val, _) = solve(&prefix, l, r, low);
    val + i64::from(m) * low
}

// (max_sum, count of subarrs)
fn solve(prefix: &[i64], l: usize, r: usize, penalty: i64) -> (i64, i32) {
    let n = prefix.len() - 1;
    let mut dp_val = vec![0; 1 + n];
    let mut dp_count = vec![0; 1 + n];
    let mut queue = VecDeque::new();
    let mut left = 0;
    for right in l..=n {
        // skip
        dp_val[right] = dp_val[right - 1];
        dp_count[right] = dp_count[right - 1];

        while left <= right - l {
            let val = dp_val[left] - prefix[left];
            let count = dp_count[left];
            while let Some(&(back_val, back_count, _)) = queue.back()
                && (back_val < val || (back_val == val && back_count <= count))
            {
                queue.pop_back();
            }
            while let Some(&(_, _, i)) = queue.front()
                && i < right.saturating_sub(r)
            {
                queue.pop_front();
            }
            queue.push_back((val, count, left));
            left += 1;
        }
        // `queue` is ensured to have element
        let Some(&(val, count, _)) = queue.front() else {
            unreachable!();
        };
        // take
        let curr_val = val + prefix[right] - penalty;
        let curr_count = 1 + count;
        if curr_val >= dp_val[right] {
            dp_val[right] = curr_val;
            dp_count[right] = dp_count[right].max(curr_count);
        }
    }
    (dp_val[n], dp_count[n])
}

fn single(prefix: &[i64], l: usize, r: usize) -> i64 {
    let n = prefix.len() - 1;
    let mut res = i64::MIN >> 2;
    let mut queue = VecDeque::<(usize, i64)>::new();
    let mut left = 0;
    for right in l..=n {
        while left <= right - l {
            let num = prefix[left];
            while let Some(&(_, val)) = queue.back()
                && val >= num
            {
                queue.pop_back(); // we want min(prefix[left])
            }
            while let Some(&(i, _)) = queue.front()
                && i < right.saturating_sub(r)
            {
                queue.pop_front();
            }
            queue.push_back((left, num));
            left += 1;
        }
        // `queue` is ensured to have element
        res = res.max(prefix[right] - queue.front().map(|&(_, v)| v).unwrap_or(0));
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
        assert_eq!(maximum_sum(&[-1, 7, -4], 1, 2, 3), 6);
        assert_eq!(maximum_sum(&[4, 1, -5, 2], 2, 1, 3), 7);
        assert_eq!(maximum_sum(&[1, 0, 3, 4], 2, 1, 2), 8);
        assert_eq!(maximum_sum(&[-3, -4, -1], 2, 1, 2), -1);
    }

    #[test]
    fn test() {}
}
