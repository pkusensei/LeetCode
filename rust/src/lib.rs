mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::HashMap;

pub fn maximum_and_sum(nums: &[i32], num_slots: i32) -> i32 {
    dfs(nums, num_slots, 0, 0, &mut HashMap::new())
}

fn dfs(
    nums: &[i32],
    slots: i32,
    mask1: i16,
    mask2: i16,
    memo: &mut HashMap<(usize, i16, i16), i32>,
) -> i32 {
    let n = nums.len();
    match nums {
        [] => 0,
        [head, tail @ ..] => {
            let k = (n, mask1, mask2);
            if let Some(&v) = memo.get(&k) {
                return v;
            }
            let mut res = 0;
            for slot in 0..slots {
                if (mask1 >> slot) & 1 == 1 && (mask2 >> slot) & 1 == 1 {
                    continue;
                }
                let mut curr = (1 + slot) & head;
                if (mask1 >> slot) & 1 == 1 {
                    curr += dfs(tail, slots, mask1, mask2 | (1 << slot), memo);
                } else {
                    curr += dfs(tail, slots, mask1 | (1 << slot), mask2, memo);
                }
                res = res.max(curr);
            }
            memo.insert(k, res);
            res
        }
    }
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
    fn basics() {
        assert_eq!(maximum_and_sum(&[1, 2, 3, 4, 5, 6], 3), 9);
        assert_eq!(maximum_and_sum(&[1, 3, 10, 4, 7, 1], 9), 24);
    }

    #[test]
    fn test() {}
}
