mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_len(n: i32, edges: &[[i32; 2]], label: &str) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut memo = vec![vec![vec![-1; n]; n]; 1 << n];
    dfs(&adj, label.as_bytes(), 0, 0, 0, &mut memo)
}

fn dfs(
    adj: &[Vec<usize>],
    s: &[u8],
    mask: usize,
    left: usize,
    right: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    let n = adj.len();
    if memo[mask][left][right] > -1 {
        return memo[mask][left][right];
    }
    let mut res = 0;
    if mask == 0 {
        for mid in 0..n {
            res = res.max(1 + dfs(adj, s, 1 << mid, mid, mid, memo));
            for &next in &adj[mid] {
                if s[mid] == s[next] {
                    res = res.max(2 + dfs(adj, s, (1 << mid) | (1 << next), mid, next, memo))
                }
            }
        }
    } else {
        for &next1 in adj[left].iter().filter(|&&v| (mask >> v) & 1 == 0) {
            for &next2 in adj[right].iter().filter(|&&v| (mask >> v) & 1 == 0) {
                if next1 != next2 && s[next1] == s[next2] {
                    res = res.max(
                        2 + dfs(
                            adj,
                            s,
                            mask | (1 << next1) | (1 << next2),
                            next1,
                            next2,
                            memo,
                        ),
                    );
                }
            }
        }
    }
    memo[mask][left][right] = res;
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
        assert_eq!(max_len(3, &[[0, 1], [1, 2]], "aba"), 3);
        assert_eq!(max_len(3, &[[0, 1], [0, 2]], "abc"), 1);
        assert_eq!(max_len(4, &[[0, 2], [0, 3], [3, 1]], "bbac"), 3);
    }

    #[test]
    fn test() {}
}
