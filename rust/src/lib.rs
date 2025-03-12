mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut dists = vec![vec![i32::MAX; cols]; rows];
    dists[0][0] = 0;
    let mut queue = std::collections::VecDeque::from([([0, 0], 0)]);
    while let Some(([r, c], dist)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 {
            return dist;
        }
        for [nr, nc] in neighbors([r, c]) {
            if let Some(&v) = grid.get(nr).and_then(|row| row.get(nc)) {
                if dists[nr][nc] < i32::MAX {
                    continue;
                }
                if v == 1 {
                    queue.push_back(([nr, nc], 1 + dist));
                    dists[nr][nc] = 1 + dist;
                } else {
                    queue.push_front(([nr, nc], dist));
                    dists[nr][nc] = dist;
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
    fn basics() {}

    #[test]
    fn test() {}
}
