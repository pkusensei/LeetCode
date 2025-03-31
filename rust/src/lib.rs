mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut idx = 0;
    let mut count = 0;
    for (r, row) in mat.iter().enumerate() {
        let sum: i32 = row.iter().sum();
        if sum > count {
            idx = r as i32;
            count = sum;
        }
    }
    vec![idx, count]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
