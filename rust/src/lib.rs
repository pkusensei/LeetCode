mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let rows: Vec<_> = grid
        .iter()
        .map(|r| r.iter().fold(0, |acc, &v| (acc << 1) | v))
        .collect();
    for (i1, v1) in rows.iter().enumerate() {
        if v1.count_ones() == 0 {
            return vec![i1 as i32];
        }
        for (i2, v2) in rows.iter().enumerate().skip(1 + i1) {
            if v1 & v2 == 0 {
                return vec![i1 as i32, i2 as i32];
            }
        }
    }
    vec![]
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
