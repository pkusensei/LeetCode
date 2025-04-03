mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let [rows, cols] = get_dimensions(&grid);
    let mut res = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let mut a = HashSet::new();
            let mut b = HashSet::new();
            let mut cr = r;
            let mut cc = c;
            while cr > 0 && cc > 0 {
                a.insert(grid[cr - 1][cc - 1]);
                cr -= 1;
                cc -= 1;
            }
            cr = r;
            cc = c;
            while cr + 1 < rows && cc + 1 < cols {
                b.insert(grid[1 + cr][1 + cc]);
                cr += 1;
                cc += 1;
            }
            res[r][c] = a.len().abs_diff(b.len()) as i32;
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
