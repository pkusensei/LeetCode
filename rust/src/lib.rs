mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(matrix: Vec<String>) -> i32 {
    use std::collections::{HashMap, VecDeque};
    let [rows, cols] = get_dimensions(&matrix);
    let mut map = HashMap::<_, Vec<_>>::new();
    let mut seen = vec![vec![false; cols]; rows];
    for (r, s) in matrix.iter().enumerate() {
        for (c, b) in s.bytes().enumerate() {
            if b.is_ascii_alphabetic() {
                map.entry(b).or_default().push([r, c]);
            }
            if b == b'#' {
                seen[r][c] = true;
            }
        }
    }
    let mut queue = VecDeque::from([(0, 0, 0)]);
    if let Some(v) = map.get(&matrix[0].as_bytes()[0]) {
        for &[r, c] in v {
            queue.push_back((r, c, 0));
            seen[r][c] = true;
        }
    } else {
        queue.push_back((0, 0, 0));
    }
    while let Some((r, c, step)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 {
            return step;
        }
        for [nr, nc] in neighbors([r, c]) {
            if seen
                .get(nr)
                .is_some_and(|rr| rr.get(nc).is_some_and(|&v| !v))
            {
                seen[nr][nc] = true;
                if let Some(v) = map.get(&matrix[nr].as_bytes()[nc]) {
                    for &[rr, cc] in v {
                        queue.push_back((rr, cc, 1 + step));
                        seen[rr][cc] = true;
                    }
                } else {
                    queue.push_back((nr, nc, 1 + step));
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
    fn basics() {}

    #[test]
    fn test() {}
}
