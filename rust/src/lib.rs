mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    use std::collections::VecDeque;
    let [n, src, dst, k] = [n, src, dst, k].map(|v| v as usize);
    let adj = flights.iter().fold(vec![vec![]; n], |mut acc, f| {
        acc[f[0] as usize].push((f[1] as usize, f[2]));
        acc
    });
    let mut queue = VecDeque::from([(src, 0, 0)]);
    let mut seen = vec![vec![i32::MAX; 2 + k]; n];
    seen[src].fill(0);
    while let Some((node, cost, step)) = queue.pop_front() {
        if cost > seen[node][step] || step > 1 + k {
            continue;
        }
        seen[node][step] = cost;
        for &(next, c) in &adj[node] {
            if 1 + step > 1 + k || seen[node][step] + c >= seen[next][1 + step] {
                continue;
            }
            seen[next][1 + step] = c + seen[node][step];
            queue.push_back((next, c + seen[node][step], 1 + step));
        }
    }
    seen[dst]
        .iter()
        .min()
        .map(|&v| if v < i32::MAX { v } else { -1 })
        .unwrap_or(-1)
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
