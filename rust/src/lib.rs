mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_path(parent: &[i32], s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &p)| {
            if p >= 0 {
                acc[p as usize].push(i);
            }
            acc
        });
    let mut res = 0;
    dfs(&adj, s, 0, &mut res);
    res
}

fn dfs(adj: &[Vec<usize>], s: &[u8], node: usize, res: &mut i32) -> i32 {
    let [mut max1, mut max2] = [0, 0];
    for &next in adj[node].iter() {
        let temp = dfs(adj, s, next, res);
        if s[node] != s[next] {
            if temp > max1 {
                max2 = max1;
                max1 = temp;
            } else if temp > max2 {
                max2 = temp;
            }
        }
    }
    *res = (*res).max(1 + max1 + max2);
    1 + max1
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
        assert_eq!(longest_path(&[-1, 0, 0, 1, 1, 2], "abacbe"), 3);
        assert_eq!(longest_path(&[-1, 0, 0, 0], "aabc"), 3);
    }

    #[test]
    fn test() {}
}
