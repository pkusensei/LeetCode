mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_enclaves(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    let (mut count, mut edge) = (0, 0);
    let mut seen = vec![vec![false; cols]; rows];
    for (y, r) in grid.iter().enumerate() {
        for (x, &v) in r.iter().enumerate() {
            if v == 1 {
                count += 1;
                if x == 0 || x == cols - 1 || y == 0 || y == rows - 1 {
                    edge += bfs(grid, x, y, &mut seen);
                }
            }
        }
    }
    count - edge
}

fn bfs(grid: &[&[i32]], x: usize, y: usize, seen: &mut [Vec<bool>]) -> i32 {
    if seen[y][x] {
        return 0;
    }
    seen[y][x] = true;
    let mut queue = std::collections::VecDeque::from([(x, y)]);
    let mut res = 0;
    while let Some((x, y)) = queue.pop_front() {
        res += 1;
        for (nx, ny) in neighbors((x, y)) {
            if grid
                .get(ny)
                .is_some_and(|r| r.get(nx).is_some_and(|&v| v == 1))
                && !seen[ny][nx]
            {
                seen[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            num_enclaves(&[&[0, 0, 0, 0], &[1, 0, 1, 0], &[0, 1, 1, 0], &[0, 0, 0, 0]]),
            3
        );
        debug_assert_eq!(
            num_enclaves(&[&[0, 1, 1, 0], &[0, 0, 1, 0], &[0, 0, 1, 0], &[0, 0, 0, 0]]),
            0
        );
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
