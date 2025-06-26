mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    if k == 1 {
        return vec![vec![0; cols]; rows];
    }
    let mut res = Vec::with_capacity(rows - k + 1);
    for r in 0..=rows - k {
        let mut curr_row = Vec::with_capacity(cols - k + 1);
        for c in 0..=cols - k {
            let mut sub = Vec::with_capacity(k * k);
            for i in 0..k {
                sub.extend(grid[r + i][c..c + k].iter().copied());
            }
            sub.sort_unstable();
            sub.dedup();
            curr_row.push(sub.windows(2).map(|w| w[1] - w[0]).min().unwrap_or(0));
        }
        res.push(curr_row);
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
