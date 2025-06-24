mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(nums: &[i32]) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let sorted = nums
        .iter()
        .copied()
        .sorted_unstable_by_key(|&v| (f(v), v))
        .collect_vec();
    if sorted == nums {
        return 0;
    }
    let map: HashMap<_, _> = sorted.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let n = nums.len();
    let mut seen = vec![false; n];
    let mut res = 0;
    for i in 0..n {
        if i == map[&nums[i]] {
            seen[i] = true;
        } else if !seen[i] {
            let mut curr = i;
            let mut size = 0;
            while !seen[curr] {
                seen[curr] = true;
                curr = map[&nums[curr]];
                size += 1;
            }
            res += (size - 1).max(0);
        }
    }
    res
}

const fn f(mut n: i32) -> i32 {
    let mut res = 0;
    while n > 0 {
        res += n % 10;
        n /= 10;
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
    fn basics() {
        assert_eq!(min_swaps(&[37, 100]), 1);
        assert_eq!(min_swaps(&[22, 14, 33, 7]), 0);
        assert_eq!(min_swaps(&[18, 43, 34, 16]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(min_swaps(&[588871835, 925173195, 162137399]), 1);
    }
}
