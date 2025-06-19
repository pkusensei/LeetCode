mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;
    const M: i64 = 1_000_000_007;
    let n = 1 + conversions.len();
    let mut queue = VecDeque::from([(0, 1)]);
    let mut adj = vec![vec![]; n];
    for c in conversions.iter() {
        let [src, dst, f] = c[..] else { unreachable!() };
        adj[src as usize].push((dst as usize, f));
    }
    let mut res = vec![0; n];
    res[0] = 1;
    while let Some((node, val)) = queue.pop_front() {
        for &(next, f) in &adj[node] {
            if res[next] == 0 {
                let curr = val * i64::from(f) % M;
                res[next] = curr as i32;
                queue.push_back((next, curr));
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
