mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn closest_meeting_node(edges: &[i32], node1: i32, node2: i32) -> i32 {
    let a = bfs(edges, node1 as _);
    let b = bfs(edges, node2 as _);
    a.into_iter()
        .zip(b)
        .enumerate()
        .filter(|&(_i, (x, y))| x > -1 && y > -1)
        .min_by_key(|&(_i, (x, y))| x.max(y))
        .map(|(i, _)| i as i32)
        .unwrap_or(-1)
}

fn bfs(edges: &[i32], start: usize) -> Vec<i32> {
    use std::collections::VecDeque;
    let n = edges.len();
    let mut res = vec![-1; n];
    let mut queue = VecDeque::from([(start, 0)]);
    res[start] = 0;
    while let Some((node, dist)) = queue.pop_front() {
        if edges[node] > -1 {
            let next = edges[node] as usize;
            if res[next] == -1 {
                res[next] = 1 + dist;
                queue.push_back((next, 1 + dist));
            }
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
