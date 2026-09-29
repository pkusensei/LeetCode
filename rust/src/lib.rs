mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    use std::collections::VecDeque;
    let [rows, cols] = get_dimensions(&grid);
    if (rows + cols - 1) & 1 == 1 || grid[0][0] == ')' {
        return false;
    }
    let max = (rows + cols) / 2;
    let mut seen = vec![vec![vec![false; 1 + max]; cols]; rows];
    seen[0][0].fill(true);
    let mut queue = VecDeque::from([(0, 0, 1)]);
    while let Some((r, c, open)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 && open == 0 {
            return true;
        }
        let nr = 1 + r;
        if nr < rows {
            let nopen = open + if grid[nr][c] == '(' { 1 } else { -1 };
            if (0..=max as i32).contains(&nopen) && !seen[nr][c][nopen as usize] {
                seen[nr][c][nopen as usize] = true;
                queue.push_back((nr, c, nopen));
            }
        }
        let nc = 1 + c;
        if nc < cols {
            let nopen = open + if grid[r][nc] == '(' { 1 } else { -1 };
            if (0..=max as i32).contains(&nopen) && !seen[r][nc][nopen as usize] {
                seen[r][nc][nopen as usize] = true;
                queue.push_back((r, nc, nopen));
            }
        }
    }
    false
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
