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

pub fn even_sum_subgraphs(nums: &[i32], edges: &[[i32; 2]]) -> i32 {
    let n = nums.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        acc[e[0] as usize].push(e[1] as usize);
        acc[e[1] as usize].push(e[0] as usize);
        acc
    });
    let full = 1 << n;
    let mut res = 0;
    for mask in 1..full {
        res += i32::from(f(&nums, &adj, mask))
    }
    res
}

fn f(nums: &[i32], adj: &[Vec<usize>], mask: i32) -> bool {
    let n = nums.len();
    let mut queue = VecDeque::new();
    let mut sum = 0;
    let mut curr = 0;
    for bit in 0..n {
        if mask & (1 << bit) > 0 {
            sum ^= nums[bit];
            if queue.is_empty() {
                curr |= 1 << bit;
                queue.push_back(bit);
            }
        }
    }
    if sum & 1 == 1 {
        return false;
    }
    while let Some(node) = queue.pop_front() {
        if curr == mask as usize {
            break;
        }
        for &next in &adj[node] {
            if mask & (1 << next) > 0 && curr & (1 << next) == 0 {
                curr |= 1 << next;
                queue.push_back(next);
            }
        }
    }
    curr == mask as usize
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
        assert_eq!(even_sum_subgraphs(&[1, 0, 1], &[[0, 1], [1, 2]]), 2);
    }

    #[test]
    fn test() {}
}
