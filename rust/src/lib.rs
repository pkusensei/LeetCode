mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_valid_path(grid: &[&[i32]]) -> bool {
    let [rows, cols] = get_dimensions(grid).map(|v| v as i32);
    let mut queue = std::collections::VecDeque::from([[0, 0]]);
    let mut seen = vec![vec![false; cols as usize]; rows as usize];
    seen[0][0] = true;
    while let Some([row, col]) = queue.pop_front() {
        if row == rows - 1 && col == cols - 1 {
            return true;
        }
        for [dr, dc] in dir(grid[row as usize][col as usize]) {
            let nr = dr + row;
            let nc = dc + col;
            if (0..rows).contains(&nr)
                && (0..cols).contains(&nc)
                && is_connected([row, col], [nr, nc], grid)
                && !seen[nr as usize][nc as usize]
            {
                seen[nr as usize][nc as usize] = true;
                queue.push_back([nr, nc]);
            }
        }
    }
    false
}

fn is_connected(prev: [i32; 2], next: [i32; 2], grid: &[&[i32]]) -> bool {
    for [dr, dc] in dir(grid[next[0] as usize][next[1] as usize]) {
        if next[0] + dr == prev[0] && next[1] + dc == prev[1] {
            return true;
        }
    }
    false
}

const fn dir(num: i32) -> [[i32; 2]; 2] {
    match num {
        1 => [[0, -1], [0, 1]],
        2 => [[1, 0], [-1, 0]],
        3 => [[0, -1], [1, 0]],
        4 => [[0, 1], [1, 0]],
        5 => [[0, -1], [-1, 0]],
        6 => [[0, 1], [-1, 0]],
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(has_valid_path(&[&[2, 4, 3], &[6, 5, 2]]));
        assert!(!has_valid_path(&[&[1, 2, 1], &[1, 2, 1]]));
        assert!(!has_valid_path(&[&[1, 1, 2]]))
    }

    #[test]
    fn test() {
        assert!(has_valid_path(&[&[4, 1], &[6, 1]]))
    }

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
