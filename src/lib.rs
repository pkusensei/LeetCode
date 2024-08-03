pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let row = matrix.len();
    let col = matrix.first().map(|v| v.len()).unwrap_or_default();
    if row * col == 0 {
        return;
    }

    let row_flag = matrix[0].iter().any(|&n| n == 0);
    let col_flag = (0..row).any(|r| matrix[r][0] == 0);

    for i in 1..row {
        for j in 1..col {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }
    for i in 1..row {
        for j in 1..col {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }
    if row_flag {
        for v in matrix[0].iter_mut() {
            *v = 0
        }
    }
    if col_flag {
        for v in matrix.iter_mut() {
            v[0] = 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut v = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
            set_zeroes(&mut v);
            debug_assert_eq!(v, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
        }
        {
            let mut v = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
            set_zeroes(&mut v);
            debug_assert_eq!(v, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
        }
    }

    #[test]
    fn test() {
        {
            let mut v = vec![vec![1, 0]];
            set_zeroes(&mut v);
            debug_assert_eq!(v, [[0, 0]]);
        }
    }
}
