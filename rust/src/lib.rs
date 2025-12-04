mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut prev = vec![f64::from(poured)];
    for row in 0..query_row as usize {
        let mut curr = vec![0.0; 2 * (row + 1) + 1];
        for col in 0..=row {
            let overflow = (prev[col] - 1.0) / 2.0;
            if overflow > 0.0 {
                curr[col] += overflow;
                curr[1 + col] += overflow;
            }
        }
        prev = curr;
    }
    prev.get(query_glass as usize)
        .map(|&v| v.min(1.0))
        .unwrap_or_default()
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {
        assert_eq!(champagne_tower(2, 1, 1), 0.5);
    }

    #[test]
    fn test() {}
}
