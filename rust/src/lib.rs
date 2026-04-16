mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let len = nums.len();
    let map = nums
        .iter()
        .enumerate()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &v)| {
            acc.entry(v).or_default().push(i);
            acc
        });
    let mut res = vec![];
    for &q in queries.iter() {
        let q = q as usize;
        let val = nums[q];
        let arr = &map[&val];
        let n = arr.len();
        if n == 1 {
            res.push(-1);
        } else {
            let pos = arr.binary_search(&q).unwrap();
            let a = (q.abs_diff(arr[(1 + pos) % n])).min(len - q.abs_diff(arr[(1 + pos) % n]));
            let b =
                (q.abs_diff(arr[(pos + n - 1) % n])).min(len - q.abs_diff(arr[(pos + n - 1) % n]));
            let curr = a.min(b);
            res.push(curr as i32);
        }
    }
    res
}

#[allow(unused_imports)]
use helper::*;

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
