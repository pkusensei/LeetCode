mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut costs = vec![vec![i32::MAX; cols]; rows];
    let mut queue = std::collections::VecDeque::from([[0, 0]]);
    costs[0][0] = 0;
    while let Some([row, col]) = queue.pop_front() {
        let dir = grid[row][col];
        for (nr, nc) in neighbors((row, col)).filter(|&(nr, nc)| nr < rows && nc < cols) {
            let cost = if row == nr {
                if nc > col {
                    i32::from(dir != 1)
                } else {
                    i32::from(dir != 2)
                }
            } else if nr > row {
                i32::from(dir != 3)
            } else {
                i32::from(dir != 4)
            };
            if costs[row][col] + cost < costs[nr][nc] {
                costs[nr][nc] = costs[row][col] + cost;
                if cost == 1 {
                    queue.push_back([nr, nc]);
                } else {
                    queue.push_front([nr, nc]);
                }
            }
        }
    }
    costs[rows - 1][cols - 1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
