mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn range_add_queries(n: i32, queries: &[[i32; 4]]) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut res = vec![vec![0; n]; n];
    for q in queries {
        let [row1, col1, row2, col2] = [0, 1, 2, 3].map(|i| q[i] as usize);
        for r in row1..=row2 {
            res[r][col1] += 1;
            if 1 + col2 < n {
                res[r][1 + col2] -= 1;
            }
        }
    }
    for r in res.iter_mut() {
        for c in 1..n {
            r[c] += r[c - 1];
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
    fn basics() {
        assert_eq!(
            range_add_queries(3, &[[1, 1, 2, 2], [0, 0, 1, 1]]),
            [[1, 1, 0], [1, 2, 1], [0, 1, 1]]
        );
    }

    #[test]
    fn test() {}
}
