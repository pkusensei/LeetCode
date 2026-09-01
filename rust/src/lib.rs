mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
    let [rows, cols] = get_dimensions(&classroom);
    let mut start = [0, 0];
    let mut litters = Vec::with_capacity(10);
    for (r, row) in classroom.iter().enumerate() {
        for (c, b) in row.bytes().enumerate() {
            if b == b'S' {
                start = [r, c];
            }
            if b == b'L' {
                litters.push([r, c]);
            }
        }
    }
    if litters.is_empty() {
        return 0;
    }
    litters.sort_unstable();
    let n = litters.len();
    let full = 1 << n;
    let mut best = vec![vec![vec![0; full]; cols]; rows];
    best[start[0]][start[1]][0] = energy;
    let mut queue = VecDeque::from([(start, 0, energy, 0)]);
    while let Some(([r, c], mask, e, step)) = queue.pop_front() {
        if mask == full - 1 {
            return step;
        }
        if e < best[r][c][mask] {
            continue;
        }
        for [nr, nc] in neighbors([r, c]) {
            if let Some(&v) = classroom.get(nr).and_then(|row| row.as_bytes().get(nc)) {
                match v {
                    b'S' | b'.' => {
                        let ne = e - 1;
                        if ne > best[nr][nc][mask] {
                            best[nr][nc][mask] = ne;
                            queue.push_back(([nr, nc], mask, ne, 1 + step));
                        }
                    }
                    b'R' => {
                        let ne = energy;
                        if energy > best[nr][nc][mask] {
                            best[nr][nc][mask] = ne;
                            queue.push_back(([nr, nc], mask, ne, 1 + step));
                        }
                    }
                    b'L' => {
                        let i = litters.partition_point(|&v| v < [nr, nc]);
                        let nmask = mask | 1 << i;
                        let ne = e - 1;
                        if nmask == full - 1 {
                            queue.push_back(([nr, nc], nmask, ne, 1 + step));
                        } else if ne > best[nr][nc][nmask] {
                            best[nr][nc][nmask] = ne;
                            queue.push_back(([nr, nc], nmask, ne, 1 + step));
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
    -1
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
