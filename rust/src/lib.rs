mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let [rows, cols] = get_dimensions(&grid);
    let mut set = std::collections::BTreeSet::new();
    for row in 0..rows {
        for col in 0..cols {
            set.insert(grid[row][col]);
            let max = col.min(cols - col - 1).min((rows - row - 1) / 2);
            for width in 0..=max {
                let curr = (0..width)
                    .map(|w| {
                        grid[row + w][col + w]
                            + grid[row + width + w][col + width - w]
                            + grid[row + 2 * width - w][col - w]
                            + grid[row + width - w][col - width + w]
                    })
                    .sum::<i32>();
                set.insert(curr);
            }
            while set.len() > 3 {
                set.pop_first();
            }
        }
    }
    set.into_iter().rev().collect()
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
