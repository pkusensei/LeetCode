mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let single = grid[1..rows - 1].iter().flat_map(|r| &r[1..cols - 1]).max();
    let mut res = single.copied().unwrap_or(i32::MIN >> 1);
    for row in grid.iter() {
        let mut min = 0;
        let mut prefix = vec![];
        for (i, &num) in row.iter().enumerate() {
            prefix.push(num + prefix.last().unwrap_or(&0));
            if i >= 2 {
                min = min.min(prefix[i - 2]);
            }
            if i >= 1 {
                res = res.max(prefix[i] - min);
            }
        }
    }
    for c in 0..cols {
        let mut min = 0;
        let mut prefix = vec![];
        for r in 0..rows {
            prefix.push(grid[r][c] + prefix.last().unwrap_or(&0));
            if r >= 2 {
                min = min.min(prefix[r - 2]);
            }
            if r >= 1 {
                res = res.max(prefix[r] - min);
            }
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
