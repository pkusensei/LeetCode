mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let [rows, cols] = get_dimensions(&grid);
        for r in 0..rows {
            for c in 0..cols {
                if (r < rows - 1 && grid[r][c] != grid[1 + r][c])
                    || (c < cols - 1 && grid[r][c] == grid[r][1 + c])
                {
                    return false;
                }
            }
        }
        true
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
