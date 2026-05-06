mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_the_box(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let [rows, cols] = get_dimensions(&grid);
    for row in grid.iter_mut() {
        let mut prefix = 0;
        for c in 0..cols {
            if row[c] == '#' {
                row[c] = '.';
                prefix += 1;
            }
            if row[c] == '*' {
                for i in 1..=prefix {
                    row[c - i] = '#';
                }
                prefix = 0;
            }
        }
        for i in 1..=prefix {
            row[cols - i] = '#';
        }
    }
    let mut res = vec![vec![' '; rows]; cols];
    for (r, row) in grid.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            res[c][rows - 1 - r] = val;
        }
    }
    res
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
