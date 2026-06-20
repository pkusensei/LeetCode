mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        acc[e[0] as usize].push(e[1] as usize);
        acc
    });
    dfs(&adj, &base_time, 0)
}

fn dfs(adj: &[Vec<usize>], base_time: &[i32], node: usize) -> i64 {
    let mut early = None;
    let mut late = None;
    for &next in &adj[node] {
        let temp = dfs(adj, base_time, next);
        late = late.max(Some(temp));
        if early.is_none_or(|v| v > temp) {
            early = Some(temp);
        }
    }
    let dur = late.zip(early).map(|(a, b)| a - b).unwrap_or(0) + i64::from(base_time[node]);
    dur + late.unwrap_or(0)
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
