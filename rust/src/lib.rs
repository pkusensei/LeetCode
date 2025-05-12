mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut a = 0;
    for row in grid.iter() {
        let mut left = 0;
        let mut right = cols - 1;
        while left < right {
            a += i32::from(row[left] != row[right]);
            left += 1;
            right -= 1;
        }
    }
    let mut b = 0;
    for c in 0..cols {
        let mut up = 0;
        let mut down = rows - 1;
        while up < down {
            b += i32::from(grid[up][c] != grid[down][c]);
            up += 1;
            down -= 1;
        }
    }
    a.min(b)
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
