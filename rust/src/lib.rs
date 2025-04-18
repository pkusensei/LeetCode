mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let mut grid = mat.clone();
    let n = grid[0].len();
    let k = k as usize % n;
    for (r, row) in grid.iter_mut().enumerate() {
        if r & 1 == 0 {
            row.rotate_left(k);
        } else {
            row.rotate_right(k);
        }
    }
    grid == mat
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
