mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    let mut set = std::collections::HashSet::from([0]);
    for num in reward_values {
        for score in set.clone() {
            if score < num {
                set.insert(num + score);
            }
        }
    }
    set.into_iter().max().unwrap()
    // let sum: i32 = reward_values.iter().sum();
    // let mut dp = vec![false; 1 + sum as usize];
    // dp[0] = true;
    // let mut res = 0;
    // for &num in reward_values.iter() {
    //     let num = num as usize;
    //     let mut curr = dp.clone();
    //     curr[num] = true;
    //     for val in num..(2 * num).min(1 + sum as usize) {
    //         curr[val] |= dp[val - num];
    //         if curr[val] {
    //             res = res.max(val);
    //         }
    //     }
    //     dp = curr;
    // }
    // res as i32
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
        assert_eq!(max_total_reward(vec![1, 1, 3, 3]), 4);
        assert_eq!(max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }

    #[test]
    fn test() {
        assert_eq!(max_total_reward(vec![6, 13, 9, 19]), 34);
    }
}
