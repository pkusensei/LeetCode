mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(nums: &[i32]) -> i32 {
    let [mut odds, mut evens] = [vec![], vec![]];
    for (i, &num) in nums.iter().enumerate() {
        if num & 1 == 1 {
            odds.push(i);
        } else {
            evens.push(i);
        }
    }
    if odds.len().abs_diff(evens.len()) > 1 {
        return -1;
    }
    let mut res = i32::MAX;
    if evens.len() >= odds.len() {
        res = res.min(f(&evens))
    }
    if odds.len() >= evens.len() {
        res = res.min(f(&odds))
    }
    res
}

fn f(arr: &[usize]) -> i32 {
    arr.iter()
        .enumerate()
        .map(|(i, &v)| v.abs_diff(2 * i) as i32)
        .sum()
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
        assert_eq!(min_swaps(&[2, 4, 6, 5, 7]), 3);
        assert_eq!(min_swaps(&[2, 4, 5, 7]), 1);
        assert_eq!(min_swaps(&[1, 2, 3]), 0);
        assert_eq!(min_swaps(&[4, 5, 6, 8]), -1);
    }

    #[test]
    fn test() {}
}
