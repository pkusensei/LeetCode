mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
    let (s, n) = (s.as_bytes(), s.len());
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &v)| {
            if v >= 0 {
                acc[v as usize].push(i);
            }
            acc
        });
    let mut seen = [n; 26];
    let mut res = vec![1; n];
    dfs(&adj, s, 0, n, &mut seen, &mut res);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    s: &[u8],
    node: usize,
    prev: usize,
    seen: &mut [usize; 26],
    res: &mut [i32],
) -> i32 {
    let n = s.len();
    let bi = usize::from(s[node] - b'a');
    let temp = seen[bi];
    seen[bi] = node;
    for &next in &adj[node] {
        if next != prev {
            res[node] += dfs(adj, s, next, node, seen, res);
        }
    }
    if temp < n && prev < n {
        res[prev] -= res[node];
        res[temp] += res[node];
    }
    seen[bi] = temp;
    res[node]
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
    fn test() {
        {
            let nums = vec![4, 12, 8, 14, 14, 12, 11, 20, 17, 17, 4, 10, 10, 18, 1, 16];
            assert_eq!(max_selected_elements(nums), 12);
        }
    }
}
