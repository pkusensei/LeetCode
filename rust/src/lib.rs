mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(&mat);
    let mut queue = std::collections::VecDeque::new();
    for y in 0..rows {
        for x in 0..cols {
            if mat[y][x] > 0 {
                mat[y][x] = -1
            } else {
                queue.push_back((y, x, 0));
            }
        }
    }
    while let Some((row, col, dist)) = queue.pop_front() {
        if mat[row][col] > 0 {
            continue;
        }
        mat[row][col] = dist;
        for (nr, nc) in
            neighbors((row, col)).filter(|&(r, c)| r < rows && c < cols && mat[r][c] < 0)
        {
            queue.push_back((nr, nc, dist + 1));
        }
    }
    mat
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
        );
        debug_assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            [[0, 0, 0], [0, 1, 0], [1, 2, 1]]
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
}
