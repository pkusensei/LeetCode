mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn solve_queries(nums: &[i32], queries: &[i32]) -> Vec<i32> {
    use std::collections::HashMap;
    let n = nums.len();
    let map = nums
        .iter()
        .enumerate()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &num)| {
            acc.entry(num).or_default().push(i);
            acc
        });
    let mut res = vec![];
    for q in queries.iter().map(|&q| q as usize) {
        let num = nums[q];
        let arr = &map[&num];
        if arr.len() < 2 {
            res.push(-1);
        } else {
            let i = arr.binary_search(&q).unwrap();
            let prev = i.checked_sub(1).unwrap_or(arr.len() - 1);
            let next = (1 + i) % arr.len();
            let a = q.abs_diff(arr[prev]);
            let b = q.abs_diff(arr[next]);
            res.push(a.min(n - a).min(b.min(n - b)) as i32);
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
