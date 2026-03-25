mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let [rows, cols] = get_dimensions(&grid);
    let mut prefix = Vec::with_capacity(rows);
    for (r, row) in grid.iter().enumerate() {
        let mut curr = row.iter().fold(vec![], |mut acc, &v| {
            acc.push(i64::from(v) + acc.last().unwrap_or(&0));
            acc
        });
        if r > 0 {
            for (c, p) in curr.iter_mut().zip(&prefix[r - 1]) {
                *c += p;
            }
        }
        prefix.push(curr);
    }
    let total = prefix[rows - 1][cols - 1];
    prefix.iter().any(|row| row[cols - 1] * 2 == total)
        || prefix[rows - 1].iter().any(|&v| v * 2 == total)
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
