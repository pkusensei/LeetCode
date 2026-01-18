mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_magic_square(grid: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut row_sums = Vec::with_capacity(rows);
    for row in grid.iter() {
        let curr = row.iter().fold(Vec::with_capacity(cols), |mut acc, v| {
            acc.push(v + acc.last().unwrap_or(&0));
            acc
        });
        row_sums.push(curr);
    }
    let mut col_sums = vec![vec![0; cols]; rows];
    for c in 0..cols {
        col_sums[0][c] = grid[0][c];
        for r in 1..rows {
            col_sums[r][c] = col_sums[r - 1][c] + grid[r][c];
        }
    }
    for len in (2..=rows.min(cols)).rev() {
        for r in 0..=(rows - len) {
            'out: for c in 0..=(cols - len) {
                let sum = row_sums[r][c + len - 1] - if c > 0 { row_sums[r][c - 1] } else { 0 };
                for rr in 1 + r..r + len {
                    let curr =
                        row_sums[rr][c + len - 1] - if c > 0 { row_sums[rr][c - 1] } else { 0 };
                    if curr != sum {
                        continue 'out;
                    }
                }
                for cc in c..c + len {
                    let curr =
                        col_sums[r + len - 1][cc] - if r > 0 { col_sums[r - 1][cc] } else { 0 };
                    if curr != sum {
                        continue 'out;
                    }
                }
                let [mut d1, mut d2] = [0, 0];
                for i in 0..len {
                    d1 += grid[r + i][c + i];
                    d2 += grid[r + i][c + len - 1 - i];
                }
                if d1 == sum && d2 == sum {
                    return len as i32;
                }
            }
        }
    }
    1
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
        assert_eq!(
            largest_magic_square(&[
                &[1, 9, 3, 5, 5, 8, 1, 6, 9],
                &[4, 1, 1, 6, 8, 3, 5, 7, 6],
                &[9, 8, 4, 7, 2, 4, 9, 2, 7],
                &[1, 9, 8, 10, 5, 10, 1, 6, 3]
            ]),
            3
        );
    }
}
