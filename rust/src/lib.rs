mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let [rows, cols] = get_dimensions(&mat);
    let prefix = build(&mat);
    let mut left = 0;
    let mut right = rows.min(cols);
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if check(threshold, rows, cols, &prefix, mid) {
            left = mid;
        } else {
            right = mid - 1
        }
    }
    left as i32
}

fn build(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let cols = grid[0].len();
    // prefix sum matrix of (1+rows)*(1+cols)
    let mut prefix = vec![vec![0; 1 + cols]];
    for (r, row) in grid.iter().enumerate() {
        let mut curr = row.iter().fold(vec![0], |mut acc, v| {
            acc.push(v + acc.last().unwrap_or(&0));
            acc
        });
        for (c, v) in curr.iter_mut().enumerate() {
            *v += prefix[r][c];
        }
        prefix.push(curr);
    }
    prefix
}

fn check(threshold: i32, rows: usize, cols: usize, prefix: &[Vec<i32>], mid: usize) -> bool {
    for r in 1..=rows - mid + 1 {
        for c in 1..=cols - mid + 1 {
            let curr = prefix[r + mid - 1][c + mid - 1]
                - prefix[r - 1][c + mid - 1]
                - prefix[r + mid - 1][c - 1]
                + prefix[r - 1][c - 1];
            if curr <= threshold {
                return true;
            }
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
    fn basics() {}

    #[test]
    fn test() {}
}
