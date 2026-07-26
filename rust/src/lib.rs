mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(m: i32, n: i32, penalty: Vec<Vec<i32>>) -> i64 {
    let [rows, cols] = [m, n].map(|v| v as usize);
    let mut dists = vec![vec![[i64::MAX >> 1; 2]; cols]; rows];
    dists[0][0][0] = 1;
    let mut heap = BinaryHeap::from([(Reverse(1), 0, 0, 0)]);
    while let Some((Reverse(cost), row, col, parity)) = heap.pop() {
        if row == rows - 1 && col == cols - 1 {
            continue;
        }
        if cost > dists[row][col][parity] {
            continue;
        }
        let nparity = parity ^ 1;
        let pen = i64::from(penalty[row][col]);
        for [nr, nc] in neighbors([row, col]) {
            if nr >= rows || nc >= cols {
                continue;
            }
            let ncost = cost
                + if nparity & 1 == 1 && (nr > row || nc > col) {
                    ((1 + nr) * (1 + nc)) as i64
                } else if nparity & 1 == 0 && (nr < row || nc < col) {
                    ((1 + nr) * (1 + nc)) as i64
                } else {
                    ((1 + nr) * (1 + nc)) as i64 + pen
                };
            if ncost < dists[nr][nc][nparity] {
                dists[nr][nc][nparity] = ncost;
                heap.push((Reverse(ncost), nr, nc, nparity));
            }
        }
        let ncost = cost + pen;
        if ncost < dists[row][col][nparity] {
            dists[row][col][nparity] = ncost;
            heap.push((Reverse(ncost), row, col, nparity));
        }
    }
    dists[rows - 1][cols - 1][0].min(dists[rows - 1][cols - 1][1])
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
