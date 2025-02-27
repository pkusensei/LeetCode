mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let start = find_start(&pairs).unwrap_or(pairs[0][0]);
    let mut adj = pairs
        .iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, p| {
            acc.entry(p[0]).or_default().push(p[1]);
            acc
        });
    let mut res = vec![];
    dfs(&mut adj, start, &mut res);
    res.reverse();
    res.windows(2).map(|w| w.to_vec()).collect()
}

fn dfs(adj: &mut HashMap<i32, Vec<i32>>, node: i32, res: &mut Vec<i32>) {
    if let Some(next) = adj.get_mut(&node).and_then(|v| v.pop()) {
        dfs(adj, next, res);
    }
    res.push(node);
}

fn find_start(pairs: &[Vec<i32>]) -> Option<i32> {
    let count = pairs.iter().fold(HashMap::new(), |mut acc, p| {
        let [start, end] = p[..] else { unreachable!() };
        *acc.entry(start).or_insert(0) += 1;
        *acc.entry(end).or_insert(0) -= 1;
        acc
    });
    count
        .iter()
        .find_map(|(k, v)| if *v == 1 { Some(*k) } else { None })
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
