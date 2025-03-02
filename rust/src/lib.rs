mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn possible_to_stamp(grid: &[&[i32]], stamp_height: i32, stamp_width: i32) -> bool {
    let [rows, cols] = get_dimensions(grid);
    let [height, width] = [stamp_height, stamp_width].map(|v| v as usize);
    // records all empty points
    let mut prefix = vec![vec![0; 1 + cols]; 1 + rows];
    // a stamp with bottom-right at [r, c]
    let mut good = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            prefix[1 + r][1 + c] =
                prefix[1 + r][c] + prefix[r][1 + c] - prefix[r][c] + (1 - grid[r][c]);
            if 1 + r >= height && 1 + c >= width {
                let y = 1 + r - height;
                let x = 1 + c - width;
                good[r][c] += i32::from(
                    prefix[1 + r][1 + c] - prefix[y][1 + c] - prefix[1 + r][x] + prefix[y][x]
                        == (width * height) as i32,
                ); // possible stamps at [r, c]
            }
        }
    }
    // The number of stamps to have [r, c] covered
    let mut covered = vec![vec![0; 1 + cols]; 1 + rows];
    for r in 0..rows {
        for c in 0..cols {
            covered[1 + r][1 + c] =
                covered[1 + r][c] + covered[r][1 + c] - covered[r][c] + good[r][c];
        }
    }
    for r in 0..rows {
        for c in 0..cols {
            let y = (r + height).min(rows);
            let x = (c + width).min(cols);
            if grid[r][c] == 0 && covered[y][x] - covered[y][c] - covered[r][x] + covered[r][c] == 0
            {
                return false; // [r, c] is not covered
            }
        }
    }
    true
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
    fn basics() {
        assert!(possible_to_stamp(
            &[
                &[1, 0, 0, 0],
                &[1, 0, 0, 0],
                &[1, 0, 0, 0],
                &[1, 0, 0, 0],
                &[1, 0, 0, 0]
            ],
            4,
            3
        ));
        assert!(!possible_to_stamp(
            &[&[1, 0, 0, 0], &[0, 1, 0, 0], &[0, 0, 1, 0], &[0, 0, 0, 1]],
            2,
            2
        ));
    }

    #[test]
    fn test() {}
}
