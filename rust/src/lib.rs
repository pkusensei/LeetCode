mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_moves(grid: [[i32; 3]; 3]) -> i32 {
    use std::collections::{HashSet, VecDeque};
    let mut board = [[0; 3]; 3];
    for (b, g) in board.iter_mut().zip(grid.iter()) {
        b.copy_from_slice(g);
    }
    let mut queue = VecDeque::from([(board, 0)]);
    let mut seen = HashSet::from([board]);
    while let Some((curr, dist)) = queue.pop_front() {
        if curr.iter().all(|r| r.iter().all(|&v| v == 1)) {
            return dist;
        }
        for (r, row) in curr.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                if v > 1 {
                    for [nr, nc] in neighbors([r, c]).filter(|&[nr, nc]| {
                        curr.get(nr)
                            .is_some_and(|rr| rr.get(nc).is_some_and(|&vv| vv < v))
                    }) {
                        let mut next = curr;
                        next[r][c] -= 1;
                        next[nr][nc] += 1;
                        if seen.insert(next) {
                            queue.push_back((next, 1 + dist));
                        }
                    }
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
        assert_eq!(minimum_moves([[1, 1, 0], [1, 1, 1], [1, 2, 1]]), 3);
        assert_eq!(minimum_moves([[1, 3, 0], [1, 0, 0], [1, 0, 3]]), 4);
    }

    #[test]
    fn test() {}
}
