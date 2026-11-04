mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
    let [x, k] = [x, k].map(|v| v as usize);
    let arr = slide(&nums, x);
    dfs(&arr, 0, x, k, &mut vec![vec![-1; 1 + k]; arr.len()])
}

fn dfs(arr: &[i64], idx: usize, x: usize, k: usize, memo: &mut [Vec<i64>]) -> i64 {
    if k == 0 {
        return 0;
    }
    if idx >= arr.len() {
        return i64::MAX >> 1;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    let skip = dfs(arr, 1 + idx, x, k, memo);
    let take = arr[idx] + dfs(arr, x + idx, x, k - 1, memo);
    memo[idx][k] = skip.min(take);
    skip.min(take)
}

fn slide(nums: &[i32], x: usize) -> Vec<i64> {
    use std::collections::BTreeMap;

    let mut res = Vec::with_capacity(nums.len() - x + 1);
    let [mut small, mut big] = [const { BTreeMap::new() }; 2];
    let [mut small_sum, mut big_sum] = [0, 0];
    let [mut small_count, mut big_count] = [0, 0];
    for (idx, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        if small.last_key_value().is_some_and(|(&k, _)| k >= num) {
            *small.entry(num).or_insert(0) += 1;
            small_sum += num;
            small_count += 1;
        } else {
            *big.entry(num).or_insert(0) += 1;
            big_sum += num;
            big_count += 1;
        }
        if idx >= x {
            let del = i64::from(nums[idx - x]);
            if let Some(v) = small.get_mut(&del) {
                *v -= 1;
                small_count -= 1;
                small_sum -= del;
                if *v == 0 {
                    small.remove(&del);
                }
            } else if let Some(v) = big.get_mut(&del) {
                *v -= 1;
                big_count -= 1;
                big_sum -= del;
                if *v == 0 {
                    big.remove(&del);
                }
            }
        }
        while small_count < big_count {
            let Some((k, v)) = big.iter_mut().next() else {
                break;
            };
            *small.entry(*k).or_insert(0) += 1;
            small_count += 1;
            small_sum += k;
            *v -= 1;
            big_count -= 1;
            big_sum -= k;
            if *v == 0 {
                big.pop_first();
            }
        }
        while small_count > 1 + big_count {
            let Some((k, v)) = small.iter_mut().last() else {
                break;
            };
            *big.entry(*k).or_insert(0) += 1;
            big_count += 1;
            big_sum += k;
            *v -= 1;
            small_count -= 1;
            small_sum -= k;
            if *v == 0 {
                small.pop_last();
            }
        }
        if idx >= x - 1 {
            let med = small.last_key_value().map(|(k, _)| *k).unwrap();
            let val = med * small_count - small_sum + big_sum - med * big_count;
            res.push(val);
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
