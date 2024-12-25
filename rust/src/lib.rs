mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path(grid: &[&[i32]], k: i32) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    let mut queue = std::collections::VecDeque::from([(0, 0, 0, k)]);
    let mut visited = vec![vec![vec![false; 1 + k as usize]; cols]; rows];
    visited[0][0][k as usize] = true;
    while let Some((row, col, dist, k)) = queue.pop_front() {
        if row == rows - 1 && col == cols - 1 {
            return dist;
        }
        for (nr, nc) in neighbors((row, col)) {
            match grid.get(nr).and_then(|r| r.get(nc)) {
                Some(0) if !visited[nr][nc][k as usize] => {
                    visited[nr][nc][k as usize] = true;
                    queue.push_back((nr, nc, 1 + dist, k));
                }
                Some(1) if k > 0 => {
                    let nk = k - 1;
                    if !visited[nr][nc][nk as usize] {
                        visited[nr][nc][nk as usize] = true;
                        queue.push_back((nr, nc, 1 + dist, nk));
                    }
                }
                _ => (),
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            shortest_path(
                &[&[0, 0, 0], &[1, 1, 0], &[0, 0, 0], &[0, 1, 1], &[0, 0, 0]],
                1
            ),
            6
        );
        assert_eq!(shortest_path(&[&[0, 1, 1], &[1, 1, 1], &[1, 0, 0]], 1), -1);
    }

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
