mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(&mat);
    let (r, c) = (r as usize, c as usize);
    if rows * cols != r * c {
        return mat;
    }
    mat.into_iter()
        .flat_map(|v| v.into_iter())
        .collect::<Vec<_>>()
        .chunks_exact(c)
        .map(|v| v.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            [[1, 2, 3, 4]]
        );
        debug_assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            [[1, 2], [3, 4]]
        )
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
