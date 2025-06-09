mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    for col in 1..n {
        let mut c = col;
        let mut curr = vec![];
        for r in 0..n {
            curr.push(grid[r][c]);
            c += 1;
            if c == n {
                break;
            }
        }
        curr.sort_unstable();
        c = col;
        for (r, num) in curr.into_iter().enumerate() {
            grid[r][c] = num;
            c += 1;
        }
    }
    for row in 0..n {
        let mut r = row;
        let mut curr = vec![];
        for c in 0..n {
            curr.push(grid[r][c]);
            r += 1;
            if r == n {
                break;
            }
        }
        curr.sort_unstable_by(|a, b| b.cmp(a));
        r = row;
        for (c, num) in curr.into_iter().enumerate() {
            grid[r][c] = num;
            r += 1;
        }
    }
    grid
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
