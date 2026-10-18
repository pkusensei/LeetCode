mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_visited_cell(grid: Vec<Vec<char>>) -> Vec<i32> {
    let [rows, cols] = get_dimensions(&grid);
    let mut seen = vec![vec![false; cols]; rows];
    seen[0][0] = true;
    let [mut row, mut col] = [0, 0];
    loop {
        let mut nr = row;
        let mut nc = col;
        match grid[row][col] {
            'L' => {
                if nc == 0 {
                    break;
                } else {
                    nc -= 1
                }
            }
            'R' => nc += 1,
            'U' => {
                if nr == 0 {
                    break;
                } else {
                    nr -= 1
                }
            }
            'D' => nr += 1,
            _ => unreachable!(),
        }
        if nr >= rows || nc >= cols || seen[nr][nc] {
            break;
        }
        row = nr;
        col = nc;
        seen[row][col] = true;
    }
    vec![row as i32, col as i32]
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
