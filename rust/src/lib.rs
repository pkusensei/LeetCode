mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_diagonal_order(mat: &[&[i32]]) -> Vec<i32> {
    let (rows, cols) = get_dimensions(mat);
    let mut res = vec![];
    for i in 0..cols {
        let (mut row, mut col) = (0, i);
        let mut curr = vec![mat[row][col]];
        while col > 0 && row < rows - 1 {
            col -= 1;
            row += 1;
            curr.push(mat[row][col]);
        }
        res.push(curr);
    }
    for i in 1..rows {
        let (mut row, mut col) = (i, cols - 1);
        let mut curr = vec![mat[row][col]];
        while col > 0 && row < rows - 1 {
            col -= 1;
            row += 1;
            curr.push(mat[row][col]);
        }
        res.push(curr);
    }
    for v in res.iter_mut().step_by(2) {
        v.reverse();
    }
    res.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_diagonal_order(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]),
            [1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
        debug_assert_eq!(find_diagonal_order(&[&[1, 2], &[3, 4]]), [1, 2, 3, 4]);
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
