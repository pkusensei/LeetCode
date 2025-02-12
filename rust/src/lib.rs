mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let n = mat.len();
    for _ in 0..4 {
        mat.reverse();
        for r in 0..n {
            for c in 0..r {
                (mat[r][c], mat[c][r]) = (mat[c][r], mat[r][c]);
            }
        }
        if mat == target {
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
