mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_submatrix(matrix: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(matrix);
    let mut prefix = vec![vec![0; cols]; rows];
    for c in 0..cols {
        for r in 0..rows {
            if matrix[r][c] == 1 {
                prefix[r][c] = 1 + if r > 0 { prefix[r - 1][c] } else { 0 };
            }
        }
    }
    let mut res = 0;
    for row in prefix.iter_mut() {
        row.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
        for (c, &v) in row.iter().enumerate() {
            if v == 0 {
                break;
            }
            res = res.max(v * (1 + c as i32));
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(largest_submatrix(&[&[0, 0, 1], &[1, 1, 1], &[1, 0, 1]]), 4);
        assert_eq!(largest_submatrix(&[&[1, 0, 1, 0, 1]]), 3);
        assert_eq!(largest_submatrix(&[&[1, 1, 0], &[1, 0, 1]]), 2);
    }

    #[test]
    fn test() {}
}
