mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
    use itertools::Itertools;
    let cols = matrix[0].len();
    let mut res = 0;
    let mut prefix = vec![0; cols];
    for row in matrix.iter() {
        let mut curr = row
            .iter()
            .zip(&prefix)
            .map(|(a, b)| if *a > 0 { a + b } else { 0 })
            .collect_vec();
        prefix.copy_from_slice(&curr);
        curr.sort_unstable_by(|a, b| b.cmp(a));
        // let mut st = vec![];
        // for (right, &val) in curr.iter().enumerate() {
        //     while st.last().is_some_and(|&top| curr[top] >= val) {
        //         st.pop();
        //     }
        //     let left = st.last().map(|&v| v as i32).unwrap_or(-1);
        //     res = res.max(val * (right as i32 - left));
        //     st.push(right);
        // }
        for (i, v) in curr.iter().enumerate() {
            res = res.max(v * (1 + i as i32))
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
