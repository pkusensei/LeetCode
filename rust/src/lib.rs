mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let [rows, cols] = get_dimensions(&grid);
    if k == 0 {
        return vec![vec![0; cols]; rows];
    }
    let mut res = vec![];
    for r in 0..=rows - k {
        let mut curr = vec![];
        for c in 0..=cols - k {
            let mut mat = vec![];
            for i in 0..k {
                mat.extend_from_slice(&grid[r + i][c..c + k]);
            }
            mat.sort_unstable();
            mat.dedup();
            curr.push(mat.windows(2).map(|w| w[1] - w[0]).min().unwrap_or(0));
        }
        res.push(curr);
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
