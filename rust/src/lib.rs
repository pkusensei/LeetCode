mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_highest_score_nodes(parents: &[i32]) -> i32 {
    let n = parents.len();
    let adj = parents
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &p)| {
            if p >= 0 {
                acc[p as usize].push(i);
            }
            acc
        });
    let mut max_score = 0;
    let mut max_count = 0;
    dfs(&adj, 0, &mut max_score, &mut max_count);
    max_count
}

fn dfs(adj: &[Vec<usize>], node: usize, max_score: &mut i64, max_count: &mut i32) -> i64 {
    let n = adj.len() as i64;
    let [subtree, score] = if adj[node].is_empty() {
        [1, n - 1]
    } else {
        let mut score = 1;
        let mut subtree = 1; // this node
        for &next in adj[node].iter() {
            let v = dfs(adj, next, max_score, max_count);
            score *= v;
            subtree += v;
        }
        score *= (n - subtree).max(1); // Empty group count as 1
        [subtree, score]
    };
    match score.cmp(max_score) {
        std::cmp::Ordering::Less => (),
        std::cmp::Ordering::Equal => *max_count += 1,
        std::cmp::Ordering::Greater => {
            *max_score = score;
            *max_count = 1;
        }
    }
    subtree
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(count_highest_score_nodes(&[-1, 2, 0, 2, 0]), 3);
        assert_eq!(count_highest_score_nodes(&[-1, 2, 0]), 2);
    }

    #[test]
    fn test() {}
}
