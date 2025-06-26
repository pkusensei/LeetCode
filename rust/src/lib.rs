mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(classroom: &[&str], energy: i32) -> i32 {
    use itertools::Itertools;
    use std::collections::{HashMap, VecDeque};
    let grid = classroom.iter().map(|s| s.as_bytes()).collect_vec();
    let [rows, cols] = get_dimensions(&grid);
    let mut l_ids = HashMap::new();
    let mut start = [0, 0];
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            match v {
                b'L' => {
                    l_ids.insert([r, c], l_ids.len());
                }
                b'S' => start = [r, c],
                _ => (),
            }
        }
    }
    let n = l_ids.len();
    let mut best_energy = vec![vec![vec![-1; 1 << n]; cols]; rows];
    let mut queue = VecDeque::from([(start, 0_usize, energy, 0)]);
    best_energy[start[0]][start[1]][0] = energy;
    while let Some(([r, c], mask, e, step)) = queue.pop_front() {
        if mask == (1 << n) - 1 {
            return step;
        }
        if e == 0 {
            continue;
        }
        for [nr, nc] in neighbors([r, c]) {
            if nr < rows && nc < cols && grid[nr][nc] != b'X' {
                let ne = if grid[nr][nc] == b'R' { energy } else { e - 1 };
                let nmask = if let Some(v) = l_ids.get(&[nr, nc]) {
                    mask | (1 << v)
                } else {
                    mask
                };
                if ne > best_energy[nr][nc][mask] {
                    best_energy[nr][nc][mask] = ne;
                    queue.push_back(([nr, nc], nmask, ne, 1 + step));
                }
            }
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
        assert_eq!(min_moves(&["S.", "XL"], 2), 2);
        assert_eq!(min_moves(&["L.S", "RXL"], 3), -1);
        assert_eq!(min_moves(&["LS", "RL"], 4), 3);
    }

    #[test]
    fn test() {
        assert_eq!(min_moves(&["SLRX", "L.LR"], 3), 5);
    }
}
