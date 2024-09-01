mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn construct2_d_array(original: &[i32], m: i32, n: i32) -> Vec<Vec<i32>> {
    if (m * n) as usize != original.len() {
        return vec![];
    }
    original
        .chunks_exact(n as usize)
        .map(|w| w.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(construct2_d_array(&[1, 2, 3, 4], 2, 2), [[1, 2], [3, 4]]);
        debug_assert_eq!(construct2_d_array(&[1, 2, 3], 1, 3), [[1, 2, 3]]);
        debug_assert!(construct2_d_array(&[1, 2], 1, 1).is_empty());
    }

    #[test]
    fn test() {}
}
