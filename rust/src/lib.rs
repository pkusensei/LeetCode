mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let [rows, cols] = get_dimensions(&mat);
    let [mut left, mut right] = [0, cols - 1];
    while left <= right {
        let mid_col = left + (right - left) / 2;
        let max_row = (0..rows).max_by_key(|&r| mat[r][mid_col]).unwrap_or(0);
        let peak = mat[max_row][mid_col];
        let left_neighbor = mid_col
            .checked_sub(1)
            .map(|c| mat[max_row][c])
            .unwrap_or(i32::MIN);
        let right_neighbor = *mat[max_row].get(1 + mid_col).unwrap_or(&i32::MIN);
        if peak > left_neighbor && peak > right_neighbor {
            return vec![max_row as i32, mid_col as i32];
        } else if peak < left_neighbor {
            right = mid_col - 1;
        } else {
            left = 1 + mid_col;
        }
    }
    vec![]
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
