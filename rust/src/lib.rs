mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    const M: i64 = 12345;
    let [rows, cols] = get_dimensions(&grid);
    let f = |mut acc: Vec<i64>, &v| {
        acc.push(i64::from(v) * acc.last().unwrap_or(&1) % M);
        acc
    };
    let prefix = grid.iter().flatten().take(rows * cols - 1).fold(vec![1], f);
    let mut suffix = grid
        .iter()
        .flatten()
        .rev()
        .take(rows * cols - 1)
        .fold(vec![1], f);
    suffix.reverse();
    let mut res = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            res[r][c] = (prefix[r * cols + c] * suffix[r * cols + c] % M) as i32
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
