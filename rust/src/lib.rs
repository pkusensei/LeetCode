mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
    let sum = grid
        .iter()
        .flat_map(|r| r.iter().map(|&v| i64::from(v)))
        .sum();
    if check(&grid, sum) {
        return true;
    }
    let mut t = transpose(&grid);
    if check(&t, sum) {
        return true;
    }
    grid.reverse();
    t.reverse();
    if check(&grid, sum) || check(&t, sum) {
        return true;
    }
    false
}

fn transpose(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(grid);
    let mut res = vec![vec![0; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            res[c][r] = grid[r][c];
        }
    }
    res
}

fn check(grid: &[Vec<i32>], sum: i64) -> bool {
    let mut seen = std::collections::HashSet::new();
    let mut top = 0;
    for (r, row) in grid.iter().enumerate() {
        for v in row.iter().map(|&v| i64::from(v)) {
            top += v;
            seen.insert(v);
        }
        let bot = sum - top;
        let d = top - bot;
        if d == 0
            || d == i64::from(grid[0][0])
            || d == i64::from(*grid[0].last().unwrap())
            || d == i64::from(row[0])
        {
            return true;
        }
        if grid[0].len() > 1 && r > 0 && seen.contains(&d) {
            return true;
        }
    }
    false
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
    fn basics() {
        assert!(can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
        assert!(can_partition_grid(vec![vec![1, 2], vec![3, 4]]));
        assert!(!can_partition_grid(vec![vec![1, 2, 4], vec![2, 3, 5]]));
        assert!(!can_partition_grid(vec![vec![4, 1, 8], vec![3, 2, 6]]));
    }

    #[test]
    fn test() {
        assert!(can_partition_grid(vec![vec![5, 5, 6, 2, 2, 2]]))
    }
}
