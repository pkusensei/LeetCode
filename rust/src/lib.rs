mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path_all_keys(grid: &[&str]) -> i32 {
    use std::collections::VecDeque;
    let [rows, cols] = get_dimensions(&grid);
    let mut keys = 0;
    let mut start = [0, 0];
    for (r, row) in grid.iter().enumerate() {
        for (c, b) in row.bytes().enumerate() {
            if b.is_ascii_lowercase() {
                keys += 1;
            }
            if b == b'@' {
                start = [r, c];
            }
        }
    }
    let mut seen = vec![vec![vec![false; 1 << keys]; cols]; rows];
    seen[start[0]][start[1]][0] = true;
    let mut queue = VecDeque::from([(start[0], start[1], 0, 0)]);
    while let Some((r, c, mask, step)) = queue.pop_front() {
        if mask == (1 << keys) - 1 {
            return step;
        }
        for [nr, nc] in neighbors([r, c]) {
            let Some(b) = grid.get(nr).and_then(|row| row.as_bytes().get(nc)) else {
                continue;
            };
            match b {
                b'.' | b'@' if !seen[nr][nc][mask] => {
                    seen[nr][nc][mask] = true;
                    queue.push_back((nr, nc, mask, 1 + step));
                }
                b if b.is_ascii_lowercase() => {
                    let nmask = mask | (1 << (b - b'a'));
                    if !seen[nr][nc][nmask] {
                        seen[nr][nc][nmask] = true;
                        queue.push_back((nr, nc, nmask, 1 + step));
                    }
                }
                b if b.is_ascii_uppercase() => {
                    if mask & (1 << (b - b'A')) == 0 {
                        continue;
                    }
                    if !seen[nr][nc][mask] {
                        seen[nr][nc][mask] = true;
                        queue.push_back((nr, nc, mask, 1 + step));
                    }
                }
                _ => (),
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
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(shortest_path_all_keys(&["@...a", ".###A", "b.BCc"]), 10);
    }
}
