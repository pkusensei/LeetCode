pub fn spiral_order(matrix: &[&[i32]]) -> Vec<i32> {
    let row_limit = matrix.len();
    let col_limit = matrix.first().map(|row| row.len()).unwrap_or_default();
    if row_limit * col_limit == 0 {
        return vec![];
    }

    let (mut row_min, mut row_max) = (0, row_limit - 1);
    let (mut col_min, mut col_max) = (0, col_limit - 1);

    let (mut row, mut col) = (0, 0);
    let mut res = vec![];
    loop {
        while col < col_max {
            res.push(matrix[row][col]);
            col += 1;
        }
        res.push(matrix[row][col]);
        if res.len() >= row_limit * col_limit {
            break;
        }

        row_min += 1;
        row = row_min;
        while row < row_max {
            res.push(matrix[row][col]);
            row += 1;
        }
        res.push(matrix[row][col]);
        if res.len() >= row_limit * col_limit {
            break;
        }

        col_max -= 1;
        col = col_max;
        while col > col_min {
            res.push(matrix[row][col]);
            col -= 1;
        }
        res.push(matrix[row][col]);
        if res.len() >= row_limit * col_limit {
            break;
        }

        row_max -= 1;
        row = row_max;
        while row > row_min {
            res.push(matrix[row][col]);
            row -= 1;
        }
        res.push(matrix[row][col]);
        if res.len() >= row_limit * col_limit {
            break;
        }

        col_min += 1;
        col = col_min
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            spiral_order(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]),
            [1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        debug_assert_eq!(
            spiral_order(&[&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12]]),
            [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        )
    }

    #[test]
    fn test() {}
}
