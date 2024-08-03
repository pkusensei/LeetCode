pub fn search_matrix(matrix: &[&[i32]], target: i32) -> bool {
    let row = matrix.len();
    let col = matrix.first().map(|v| v.len()).unwrap_or_default();
    if row * col == 0 {
        return false;
    }
    let first_col: Vec<i32> = matrix.iter().map(|v| v[0]).collect();
    match first_col.binary_search(&target) {
        Ok(_) => true,
        Err(0) => false, // smaller than first element
        Err(idx) => matrix[idx - 1].binary_search(&target).is_ok(),
        // bigger then matrix[idx-1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(search_matrix(
            &[&[1, 3, 5, 7], &[10, 11, 16, 20], &[23, 30, 34, 60]],
            3
        ));
        debug_assert!(!search_matrix(
            &[&[1, 3, 5, 7], &[10, 11, 16, 20], &[23, 30, 34, 60]],
            14
        ));
    }

    #[test]
    fn test() {}
}
