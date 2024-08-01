pub fn rotate(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();
    // transpose
    for i in 0..n {
        for j in 0..i {
            swap(matrix, (i, j), (j, i));
        }
    }
    // reverse
    for i in 0..n {
        for j in 0..n / 2 {
            swap(matrix, (i, j), (i, n - 1 - j))
        }
    }
}

fn swap(matrix: &mut [Vec<i32>], (row1, col1): (usize, usize), (row2, col2): (usize, usize)) {
    let temp = matrix[row1][col1];
    matrix[row1][col1] = matrix[row2][col2];
    matrix[row2][col2] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            rotate(&mut v);
            debug_assert_eq!(v, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
        }
        {
            let mut v = vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ];
            rotate(&mut v);
            debug_assert_eq!(
                v,
                [
                    [15, 13, 2, 5],
                    [14, 3, 4, 1],
                    [12, 6, 8, 9],
                    [16, 7, 10, 11]
                ]
            )
        }
    }

    #[test]
    fn test() {}
}
