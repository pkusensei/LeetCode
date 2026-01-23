mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    use itertools::Itertools;
    use std::cmp::Reverse;
    let n = hand.len();
    let sz = group_size as usize;
    if n % sz > 0 {
        return false;
    }
    let mut freq = hand.iter().copied().counts();
    hand.sort_unstable_by_key(|&v| Reverse(v));
    while let Some(last) = hand.pop() {
        match freq.get(&last) {
            None | Some(&0) => continue,
            Some(&f) => {
                for d in 0..group_size {
                    if let Some(v) = freq.get_mut(&(last + d))
                        && *v >= f
                    {
                        *v -= f;
                    } else {
                        return false;
                    };
                }
            }
        }
    }
    true
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
