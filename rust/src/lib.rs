mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn weighted_sum(parent: Vec<i32>, nums: Vec<i32>) -> i64 {
    use std::collections::VecDeque;
    let n = parent.len();
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &v)| {
            if v >= 0 {
                acc[v as usize].push(i);
            }
            acc
        });
    let mut queue = VecDeque::from([(0, 1)]);
    let mut arr = vec![0; n];
    let mut maxd = 1;
    while let Some((node, depth)) = queue.pop_front() {
        arr[node] = depth;
        maxd = maxd.max(depth);
        for &next in &adj[node] {
            queue.push_back((next, 1 + depth));
        }
    }
    let mut res = 0;
    for (&d, &num) in arr.iter().zip(&nums) {
        let [d, num] = [d, num].map(i64::from);
        res += num * (i64::from(maxd) - d + 1);
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
    fn basics() {}

    #[test]
    fn test() {}
}
