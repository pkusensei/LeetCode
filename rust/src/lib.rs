mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximal_path_quality(values: &[i32], edges: &[[i32; 3]], max_time: i32) -> i32 {
    let n = values.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    let mut res = 0;
    let mut seen = vec![false; n];
    seen[0] = true;
    backtrack(values, &adj, 0, max_time, values[0], &mut seen, &mut res);
    res
}

fn backtrack(
    values: &[i32],
    adj: &[Vec<(usize, i32)>],
    node: usize,
    time: i32,
    val: i32,
    seen: &mut [bool],
    res: &mut i32,
) {
    if time < 0 {
        return;
    }
    if node == 0 {
        *res = (*res).max(val);
    }
    for &(next, t) in adj[node].iter() {
        if seen[next] {
            backtrack(values, adj, next, time - t, val, seen, res);
        } else {
            seen[next] = true;
            backtrack(values, adj, next, time - t, val + values[next], seen, res);
            seen[next] = false;
        }
    }
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
        assert_eq!(
            maximal_path_quality(&[0, 32, 10, 43], &[[0, 1, 10], [1, 2, 15], [0, 3, 10]], 49),
            75
        );
        assert_eq!(
            maximal_path_quality(&[5, 10, 15, 20], &[[0, 1, 10], [1, 2, 10], [0, 3, 10]], 30),
            25
        );
        assert_eq!(
            maximal_path_quality(
                &[1, 2, 3, 4],
                &[[0, 1, 10], [1, 2, 11], [2, 3, 12], [1, 3, 13]],
                50
            ),
            7
        );
    }

    #[test]
    fn test() {}
}
