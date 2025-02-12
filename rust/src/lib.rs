mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut res = 1;
    for r in 0..rows {
        for c in 0..cols {
            let side = (rows - r).min(cols - c);
            for len in 1..=side {
                let up: i32 = grid[r][c..c + len].iter().sum();
                if (1 + r..r + len).any(|i| grid[i][c..c + len].iter().sum::<i32>() != up) {
                    continue;
                }
                if (c..c + len).any(|b| (r..r + len).map(|a| grid[a][b]).sum::<i32>() != up) {
                    continue;
                }
                let d1: i32 = (r..r + len).zip(c..c + len).map(|(a, b)| grid[a][b]).sum();
                let d2: i32 = (r..r + len)
                    .zip((c..c + len).rev())
                    .map(|(a, b)| grid[a][b])
                    .sum();
                if d1 == up && d2 == up {
                    res = res.max(len);
                }
            }
        }
    }
    res as _
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
