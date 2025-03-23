mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_points(grid: &[&[i32]], queries: &[i32]) -> Vec<i32> {
    use itertools::Itertools;
    use std::{cmp::Reverse, collections::BinaryHeap};
    let [rows, cols] = get_dimensions(grid);
    // (query_idx, val)
    let queries = queries
        .iter()
        .copied()
        .enumerate()
        .sorted_unstable_by_key(|&(_i, v)| v)
        .collect_vec();
    let mut res = vec![0; queries.len()];
    let mut heap = BinaryHeap::from([(Reverse(grid[0][0]), 0, 0)]);
    let mut seen = vec![vec![false; cols]; rows];
    seen[0][0] = true;
    let mut count = 0;
    for (idx, q) in queries {
        while heap.peek().is_some_and(|&(Reverse(v), ..)| v < q) {
            let (_, r, c) = heap.pop().unwrap();
            count += 1;
            for [nr, nc] in neighbors([r, c]) {
                if nr < rows && nc < cols && !seen[nr][nc] {
                    seen[nr][nc] = true;
                    heap.push((Reverse(grid[nr][nc]), nr, nc));
                }
            }
        }
        res[idx] = count;
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
    fn basics() {
        assert_eq!(
            max_points(&[&[1, 2, 3], &[2, 5, 7], &[3, 5, 1]], &[5, 6, 2]),
            [5, 8, 1]
        );
        assert_eq!(max_points(&[&[5, 2, 1], &[1, 1, 2]], &[3]), [0]);
    }

    #[test]
    fn test() {}
}
