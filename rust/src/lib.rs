mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let [rows, cols] = get_dimensions(&grid);
    let mut prefix: Vec<Vec<i64>> = Vec::with_capacity(rows);
    for row in &grid {
        let mut curr = row.iter().fold(Vec::with_capacity(cols), |mut acc, &v| {
            acc.push(i64::from(v) + acc.last().unwrap_or(&0));
            acc
        });
        if let Some(prev) = prefix.last() {
            for (v, p) in curr.iter_mut().zip(prev) {
                *v += p;
            }
        }
        prefix.push(curr);
    }
    let sum = prefix[rows - 1][cols - 1];
    for r in 0..rows - 1 {
        if 2 * prefix[r][cols - 1] == sum {
            return true;
        }
    }
    for c in 0..cols - 1 {
        if 2 * prefix[rows - 1][c] == sum {
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
    fn basics() {}

    #[test]
    fn test() {}
}
