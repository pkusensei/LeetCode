mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn good_subtree_sum(vals: &[i32], par: &[i32]) -> i32 {
    let n = vals.len();
    let adj = par
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &v)| {
            if v >= 0 {
                acc[v as usize].push(i);
            }
            acc
        });
    dfs(&adj, &vals, 0).1
}

fn dfs(adj: &[Vec<usize>], vals: &[i32], node: usize) -> (HashMap<i32, i32>, i32) {
    const M: i32 = 1_000_000_007;
    let curr_mask = to_mask(vals[node]);
    let curr_val = if curr_mask > 0 { vals[node] } else { 0 };
    let mut dp = HashMap::from([(curr_mask, curr_val), (0, 0)]);
    let mut res = 0;
    for &next in &adj[node] {
        let (ndp, nres) = dfs(adj, vals, next);
        res = (res + nres) % M;
        for (nmask, nv) in ndp {
            for (mask, val) in dp.clone() {
                if mask & nmask == 0 {
                    let v = dp.entry(mask | nmask).or_insert(val + nv);
                    *v = (*v).max(nv + val);
                }
            }
        }
    }
    res = (res + dp.values().max().unwrap_or(&0)) % M;
    (dp, res)
}

const fn to_mask(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        let d = num % 10;
        if res & (1 << d) == 0 {
            res |= 1 << d
        } else {
            return 0;
        }
        num /= 10;
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
        assert_eq!(good_subtree_sum(&[2, 3], &[-1, 0]), 8);
        assert_eq!(good_subtree_sum(&[1, 5, 2], &[-1, 0, 0]), 15);
        assert_eq!(good_subtree_sum(&[34, 1, 2], &[-1, 0, 1]), 42);
        assert_eq!(good_subtree_sum(&[3, 22, 5], &[-1, 0, 1]), 18);
    }

    #[test]
    fn test() {}
}
