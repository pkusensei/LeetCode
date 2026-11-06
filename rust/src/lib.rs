mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_activation_cost(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut min_edge = vec![i64::MAX >> 2; n];
    let mut res = i64::MAX >> 2;
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        let [pa, pb] = [a, b].map(|i| values[i].count_ones() & 1);
        let cost = i64::from(e[2]);
        if pa == pb {
            res = res.min(cost);
        } else {
            let zero = if pa == 0 { a } else { b };
            res = res.min(min_edge[zero] + cost);
            min_edge[zero] = min_edge[zero].min(cost);
        }
    }
    i32::try_from(res).unwrap_or(-1)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
