mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0];
    for row in triangle {
        let mut curr = vec![i32::MAX; row.len()];
        for (i, num) in row.iter().enumerate() {
            if let Some(prev) = dp.get(i) {
                curr[i] = curr[i].min(prev + num);
            }
            if let Some(prev) = i.checked_sub(1).and_then(|i| dp.get(i)) {
                curr[i] = curr[i].min(prev + num);
            }
        }
        dp = curr;
    }
    dp.into_iter().min().unwrap()
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
    fn basics() {}

    #[test]
    fn test() {}
}
