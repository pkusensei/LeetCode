mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn search_matrix(matrix: &[&[i32]], target: i32) -> bool {
    let first_col: Vec<_> = matrix.iter().map(|r| r[0]).collect();
    match first_col.binary_search(&target) {
        Ok(_) => true,
        Err(y) if y == 0 => false,
        Err(y1) => {
            let last_col: Vec<_> = matrix.iter().map(|r| *r.last().unwrap()).take(y1).collect();
            match last_col.binary_search(&target) {
                Ok(_) => true,
                Err(y2) => (y2..y1).any(|y| matrix[y].binary_search(&target).is_ok()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(search_matrix(
            &[
                &[1, 4, 7, 11, 15],
                &[2, 5, 8, 12, 19],
                &[3, 6, 9, 16, 22],
                &[10, 13, 14, 17, 24],
                &[18, 21, 23, 26, 30]
            ],
            5
        ));
        debug_assert!(!search_matrix(
            &[
                &[1, 4, 7, 11, 15],
                &[2, 5, 8, 12, 19],
                &[3, 6, 9, 16, 22],
                &[10, 13, 14, 17, 24],
                &[18, 21, 23, 26, 30]
            ],
            20
        ))
    }

    #[test]
    fn test() {}
}
