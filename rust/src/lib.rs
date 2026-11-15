mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(parent: Vec<i32>, op: Vec<i32>, score: Vec<i32>) -> i32 {
    let n = parent.len();
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &p)| {
            if p >= 0 {
                acc[p as usize].push(i);
            }
            acc
        });
    dfs(&adj, &op, &score, 0, n)
}

fn dfs(adj: &[Vec<usize>], op: &[i32], score: &[i32], node: usize, prev: usize) -> i32 {
    if adj[node].is_empty() {
        return score[node];
    }
    let mut sum = score[node];
    let mut min = i32::MAX >> 1;
    for &next in &adj[node] {
        if next != prev {
            let sub = dfs(adj, op, score, next, node);
            sum += sub;
            min = min.min(sub);
        }
    }
    if op[node] == 1 || min < 0 {
        sum - min
    } else {
        sum
    }
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
    fn basics() {
        {
            let parent: Vec<i32> = vec![-1, 0, 0, 0, 1, 2, 3, 3];
            let op: Vec<i32> = vec![1, 1, 0, 0, 1, 1, 0, 1];
            let score: Vec<i32> = vec![5, 6, 1, 2, 3, 2, 3, -3];
            assert_eq!(max_score(parent, op, score), 16);
        }
    }

    #[test]
    fn test() {}
}
