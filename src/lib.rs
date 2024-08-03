pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let (mut row_min, mut row_max) = (0, n as usize - 1);
    let (mut col_min, mut col_max) = (0, n as usize - 1);

    let (mut row, mut col) = (0, 0);
    let mut res = vec![vec![0; n as usize]; n as usize];
    let mut curr = 1;
    loop {
        while col < col_max {
            res[row][col] = curr;
            col += 1;
            curr += 1
        }
        res[row][col] = curr;
        curr += 1;
        if curr > n * n {
            break;
        }

        row_min += 1;
        row = row_min;
        while row < row_max {
            res[row][col] = curr;
            curr += 1;
            row += 1;
        }
        res[row][col] = curr;
        curr += 1;
        if curr > n * n {
            break;
        }

        col_max -= 1;
        col = col_max;
        while col > col_min {
            res[row][col] = curr;
            curr += 1;
            col -= 1;
        }
        res[row][col] = curr;
        curr += 1;
        if curr > n * n {
            break;
        }

        row_max -= 1;
        row = row_max;
        while row > row_min {
            res[row][col] = curr;
            curr += 1;
            row -= 1;
        }
        res[row][col] = curr;
        curr += 1;
        if curr > n * n {
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
        debug_assert_eq!(generate_matrix(3), [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
        debug_assert_eq!(generate_matrix(1), [[1]]);
    }

    #[test]
    fn test() {}
}
