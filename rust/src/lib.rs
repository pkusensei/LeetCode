mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut res = 0;
    for r in 0..rows / 2 {
        for c in 0..cols / 2 {
            let curr = grid[r][c]
                + grid[rows - 1 - r][c]
                + grid[r][cols - 1 - c]
                + grid[rows - 1 - r][cols - 1 - c];
            res += curr.min(4 - curr);
        }
    }
    let mut changes = 0;
    let mut one_paris = 0;
    if rows & 1 == 1 {
        let row = &grid[rows / 2];
        for (&a, &b) in row.iter().zip(row.iter().rev()).take(cols / 2) {
            res += a ^ b;
            changes += a ^ b;
            one_paris += (a + b) >> 1;
        }
    }
    if cols & 1 == 1 {
        for r in 0..rows / 2 {
            let a = grid[r][cols / 2];
            let b = grid[rows - 1 - r][cols / 2];
            res += a ^ b;
            changes += a ^ b;
            one_paris += (a + b) >> 1;
        }
    }
    if one_paris & 1 == 1 && changes == 0 {
        res += 2;
    }
    if rows & 1 == 1 && cols & 1 == 1 {
        res += grid[rows / 2][cols / 2]
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
