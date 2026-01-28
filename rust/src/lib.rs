mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(grid: &[&[i32]], k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let map = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &v)| (r, c, v)))
        .fold(BTreeMap::<_, Vec<_>>::new(), |mut acc, (r, c, v)| {
            acc.entry(v).or_default().push([r, c]);
            acc
        });
    let k = k as usize;
    let mut dists = vec![vec![vec![i32::MAX >> 1; 1 + k]; cols]; rows];
    dists[0][0][k] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0, 0, k)]);
    let mut max_expanded = vec![-1; k];
    while let Some((Reverse(cost), r, c, rem_k)) = heap.pop() {
        if cost > dists[r][c][rem_k] {
            continue;
        }
        if r == rows - 1 && c == cols - 1 {
            return cost;
        }
        for [nr, nc] in [[1 + r, c], [r, 1 + c]] {
            if let Some(v) = grid.get(nr).and_then(|row| row.get(nc)) {
                let ncost = cost + v;
                if ncost < dists[nr][nc][rem_k] {
                    dists[nr][nc][rem_k] = ncost;
                    heap.push((Reverse(ncost), nr, nc, rem_k));
                }
            }
        }
        // Tracks max grid value before reaching `nk` teleports
        // Suppose this cell has value `v1`, we could jump to all `v<=v1`
        // Later when seeing `v2<=v1`, no jump is useful (at same `nk`)
        if let Some(nk) = rem_k.checked_sub(1)
            && max_expanded[nk] < grid[r][c]
        {
            let start = 1 + max_expanded[nk];
            for (_, pos) in map.range(start..=grid[r][c]) {
                for &[nr, nc] in pos {
                    if cost < dists[nr][nc][nk] {
                        dists[nr][nc][nk] = cost;
                        heap.push((Reverse(cost), nr, nc, nk));
                    }
                }
            }
            max_expanded[nk] = grid[r][c];
        }
    }
    -1
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
        assert_eq!(min_cost(&[&[1, 3, 3], &[2, 5, 4], &[4, 3, 5]], 2), 7);
        assert_eq!(min_cost(&[&[1, 2], &[2, 3], &[3, 4]], 1), 9);
        assert_eq!(
            min_cost(
                &[
                    &[3, 0, 0],
                    &[3, 1, 2],
                    &[4, 4, 3],
                    &[5, 4, 6],
                    &[8, 6, 4],
                    &[9, 8, 6],
                    &[11, 8, 8],
                    &[11, 7, 12],
                    &[12, 10, 9]
                ],
                6
            ),
            23
        );
    }

    #[test]
    fn test() {}
}
