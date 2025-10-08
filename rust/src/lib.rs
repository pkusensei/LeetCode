mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    use itertools::Itertools;
    potions.sort_unstable_by(|a, b| b.cmp(a));
    let mut res = vec![0; spells.len()];
    let mut pi = 0;
    for (idx, &sp) in spells
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|&(_, &v)| v)
    {
        while potions
            .get(pi)
            .is_some_and(|&p| i64::from(p) * i64::from(sp) >= success)
        {
            pi += 1;
        }
        res[idx] = pi as i32;
    }
    res
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
