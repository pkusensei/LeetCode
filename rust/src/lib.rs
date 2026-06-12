mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use crate::binary_lifting::BinaryLifting;

pub fn assign_edge_weights(edges: &[[i32; 2]], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let bl = BinaryLifting::new(&adj, 0);
    queries
        .iter()
        .map(|q| {
            if q[0] == q[1] {
                return 0;
            }
            let dist = bl.distance(q[0] as usize - 1, q[1] as usize - 1);
            mod_pow(2, dist - 1) as i32
        })
        .collect()
}

const fn mod_pow(b: i64, exp: i32) -> i64 {
    const M: i64 = 1_000_000_007;
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 1 {
        mod_pow(b * b % M, exp >> 1) * b % M
    } else {
        mod_pow(b * b % M, exp >> 1)
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
        assert_eq!(assign_edge_weights(&[[1, 2]], &[[1, 1], [1, 2]]), [0, 1]);
    }

    #[test]
    fn test() {}
}
