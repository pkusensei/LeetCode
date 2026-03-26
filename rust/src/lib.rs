mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn tallest_billboard(rods: &[i32]) -> i32 {
    use std::collections::HashMap;
    let mut prev = HashMap::from([(0, 0)]);
    for &num in rods.iter() {
        let mut curr = prev.clone(); // skip current
        for (diff, v) in prev {
            let temp = curr.entry(diff + num).or_insert(v + num);
            *temp = (*temp).max(v + num);
            let temp = curr.entry((diff - num).abs()).or_insert(v + num);
            *temp = (*temp).max(v + num);
        }
        prev = curr;
    }
    prev.get(&0).map(|v| v / 2).unwrap_or(0)
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
        assert_eq!(tallest_billboard(&[3, 3]), 3);
    }

    #[test]
    fn test() {}
}
