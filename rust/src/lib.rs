mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn restore_array(adjacent_pairs: &[[i32; 2]]) -> Vec<i32> {
    let adj = adjacent_pairs
        .iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, e| {
            acc.entry(e[0]).or_default().push(e[1]);
            acc.entry(e[1]).or_default().push(e[0]);
            acc
        });
    let Some(start) = adj
        .iter()
        .find_map(|(&k, v)| if v.len() == 1 { Some(k) } else { None })
    else {
        return vec![];
    };
    let mut res = vec![start];
    dfs(&adj, start, None, &mut res);
    res
}

fn dfs(adj: &HashMap<i32, Vec<i32>>, node: i32, prev: Option<i32>, res: &mut Vec<i32>) {
    for &next in adj[&node].iter() {
        if prev.is_none_or(|v| v != next) {
            res.push(next);
            dfs(adj, next, Some(node), res);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(restore_array(&[[2, 1], [3, 4], [3, 2]]), [1, 2, 3, 4]);
    }

    #[test]
    fn test() {}
}
