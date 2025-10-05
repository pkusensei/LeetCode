mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&heights);
    let mut pset = HashSet::new();
    let mut aset = HashSet::new();
    for r in 0..rows {
        bfs(&heights, &mut pset, [r, 0]);
        bfs(&heights, &mut aset, [r, cols - 1]);
    }
    for c in 0..cols {
        bfs(&heights, &mut pset, [0, c]);
        bfs(&heights, &mut aset, [rows - 1, c]);
    }
    pset.intersection(&aset)
        .map(|v| vec![v[0] as i32, v[1] as i32])
        .collect()
}

fn bfs(grid: &[Vec<i32>], seen: &mut HashSet<Coord>, start: Coord) {
    if !seen.insert(start) {
        return;
    }
    let mut queue = VecDeque::from([start]);
    while let Some([r, c]) = queue.pop_front() {
        for [nr, nc] in neighbors([r, c]) {
            if let Some(&v) = grid.get(nr).and_then(|row| row.get(nc))
                && v >= grid[r][c]
                && seen.insert([nr, nc])
            {
                queue.push_back([nr, nc]);
            }
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
