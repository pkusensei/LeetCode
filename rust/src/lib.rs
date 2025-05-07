mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    // let mut set = std::collections::HashSet::from([0]);
    // for num in reward_values {
    //     for score in set.clone() {
    //         if score < num {
    //             set.insert(num + score);
    //         }
    //     }
    // }
    // set.into_iter().max().unwrap()
    let n = reward_values.len();
    let max = reward_values[n - 1];
    let mut dp = vec![0; max as usize];
    for (idx, &val) in reward_values.iter().enumerate() {
        let temp = val as usize; // With this value
        if idx.checked_sub(1).is_none_or(|v| v != temp) {
            // Find all previous processed values and their subset sums
            let limit = temp.min(max as usize - temp);
            for left in 0..limit {
                let prev = dp[left] as usize;
                dp[temp + prev] = (temp + prev) as i32;
            }
        }
    }
    max + dp.iter().max().unwrap()
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
