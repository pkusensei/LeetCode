mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_submatrix(matrix: &[&[i32]], k: i32) -> i32 {
    let (rows, cols) = get_dimensions(matrix);
    let prefix = NumMatrix::new(matrix);
    let mut res = i32::MIN;
    for row1 in 0..rows {
        for col1 in 0..cols {
            for row2 in row1..rows {
                for col2 in col1..cols {
                    let v = prefix.sum_region(row1, col1, row2, col2);
                    if v == k {
                        return k;
                    }
                    if v <= k {
                        res = res.max(v)
                    }
                }
            }
        }
    }
    res
}

#[derive(Debug, Clone)]
struct NumMatrix {
    prefix: Vec<Vec<i32>>,
}

// LC304
impl NumMatrix {
    // https://www.researchgate.net/profile/Michael-Jones-66/publication/3940582_Rapid_Object_Detection_using_a_Boosted_Cascade_of_Simple_Features/links/0f31753b419c639337000000/Rapid-Object-Detection-using-a-Boosted-Cascade-of-Simple-Features.pdf
    // 2.1 Integral Image
    fn new<T: AsRef<[i32]>>(matrix: &[T]) -> Self {
        let (rows, cols) = get_dimensions(&matrix);
        let mut row_sum = vec![vec![0; cols]; rows + 1];
        let mut prefix = vec![vec![0; cols + 1]; rows + 1];
        for (y, row) in matrix.iter().enumerate() {
            for (x, &num) in row.as_ref().iter().enumerate() {
                row_sum[y + 1][x] = row_sum[y][x] + num;
                prefix[y + 1][x + 1] = prefix[y + 1][x] + row_sum[y + 1][x];
            }
        }
        Self { prefix }
    }

    fn sum_region(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> i32 {
        let (row2, col2) = (row2 + 1, col2 + 1);
        self.prefix[row2][col2] + self.prefix[row1][col1]
            - (self.prefix[row1][col2] + self.prefix[row2][col1])
        //  In prefix sum matrix
        //                  col1
        //                   v
        //          0  0  0  0  0  ...
        //          0  n  n  n  n  ...
        // row1 ->  0  n  n [*] n  ...
        //          0  n  n  n [c]  ...
        //          0  n  n  n  n  ...
        // Starred point [*] corresponds to
        // (row1, col1) in original matrix i.e [c]
        // Notice that prefix matrix has extra 0s
        // (row2, col2) needs +1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(max_sum_submatrix(&[&[1, 0, 1], &[0, -2, 3]], 2), 2);
        debug_assert_eq!(max_sum_submatrix(&[&[2, 2, -1]], 3), 3);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
