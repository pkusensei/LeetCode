mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let [rows, cols] = get_dimensions(&height_map);
    let mut seen = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();
    for (r, row) in height_map.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if r == 0 || c == 0 || r == rows - 1 || c == cols - 1 {
                seen[r][c] = true;
                heap.push((Reverse(v), r, c));
            }
        }
    }
    let mut res = 0;
    let mut max_h = 0;
    while let Some((Reverse(curr), r, c)) = heap.pop() {
        max_h = max_h.max(curr);
        for [nr, nc] in neighbors([r, c]).filter(|&[nr, nc]| nr < rows && nc < cols) {
            if !seen[nr][nc] {
                seen[nr][nc] = true;
                let h = height_map[nr][nc];
                heap.push((Reverse(h), nr, nc));
                res += (max_h - h).max(0);
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
