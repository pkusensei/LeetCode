mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let [rows, cols] = get_dimensions(&maze);
    let mut seen = vec![vec![false; cols]; rows];
    let start = [entrance[0] as usize, entrance[1] as usize];
    let mut queue = std::collections::VecDeque::from([(start, 0)]);
    seen[start[0]][start[1]] = true;
    while let Some(([row, col], step)) = queue.pop_front() {
        if [row, col] != start && (row == 0 || row == rows - 1 || col == 0 || col == cols - 1) {
            return step;
        }
        for [nr, nc] in neighbors([row, col]).filter(|&[nr, nc]| {
            maze.get(nr)
                .is_some_and(|r| r.get(nc).is_some_and(|&v| v == '.'))
        }) {
            if !seen[nr][nc] {
                seen[nr][nc] = true;
                queue.push_back(([nr, nc], 1 + step));
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
