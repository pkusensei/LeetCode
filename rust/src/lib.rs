mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let cols = grid[0].len();
    let mut res = 0;
    let mut prev = vec![0; cols];
    for row in grid.iter() {
        let mut curr = row.iter().fold(vec![], |mut acc, v| {
            acc.push(v + acc.last().unwrap_or(&0));
            acc
        });
        for (v, p) in curr.iter_mut().zip(&prev) {
            *v += p;
            res += i32::from(*v <= k);
        }
        prev = curr;
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
