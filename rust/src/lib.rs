mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_trailing_zeros(grid: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    let mut left = vec![vec![[0, 0]; 1 + cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let [a, b] = factors(grid[r][c]);
            left[r][1 + c][0] = a + left[r][c][0];
            left[r][1 + c][1] = b + left[r][c][1];
        }
    }
    let mut top = vec![vec![[0, 0]; cols]; 1 + rows];
    for c in 0..cols {
        for r in 0..rows {
            let [a, b] = factors(grid[r][c]);
            top[1 + r][c][0] = a + top[r][c][0];
            top[1 + r][c][1] = b + top[r][c][1];
        }
    }
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            let h1 = left[r][c];
            let h2 = [
                left[r][cols][0] - left[r][1 + c][0],
                left[r][cols][1] - left[r][1 + c][1],
            ];
            let v1 = top[1 + r][c];
            let v2 = [
                top[rows][c][0] - top[r][c][0],
                top[rows][c][1] - top[r][c][1],
            ];
            res = res.max((h1[0] + v1[0]).min(h1[1] + v1[1]));
            res = res.max((h1[0] + v2[0]).min(h1[1] + v2[1]));
            res = res.max((h2[0] + v1[0]).min(h2[1] + v1[1]));
            res = res.max((h2[0] + v2[0]).min(h2[1] + v2[1]));
        }
    }
    res
}

const fn factors(mut num: i32) -> [i32; 2] {
    let [mut a, mut b] = [0, 0];
    while num > 0 && num % 2 == 0 {
        num /= 2;
        a += 1;
    }
    while num > 0 && num % 5 == 0 {
        num /= 5;
        b += 1;
    }
    [a, b]
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
        assert_eq!(
            max_trailing_zeros(&[
                &[23, 17, 15, 3, 20],
                &[8, 1, 20, 27, 11],
                &[9, 4, 6, 2, 21],
                &[40, 9, 1, 10, 6],
                &[22, 7, 4, 5, 3]
            ]),
            3
        );
        assert_eq!(max_trailing_zeros(&[&[4, 3, 2], &[7, 6, 1], &[8, 8, 8]]), 0);
    }

    #[test]
    fn test() {}
}
