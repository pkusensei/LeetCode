mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32], x: i32, k: i32) -> i64 {
    let [x, k] = [x, k].map(|v| v as usize);
    let ops = get_min_ops(&nums, x);
    dfs(&ops, x, k, 0, &mut vec![vec![-1; ops.len()]; 1 + k])
}

fn dfs(ops: &[i64], x: usize, k: usize, idx: usize, memo: &mut [Vec<i64>]) -> i64 {
    if k == 0 {
        return 0;
    }
    if idx >= ops.len() {
        return i64::MAX >> 1;
    }
    if memo[k][idx] > -1 {
        return memo[k][idx];
    }
    let skip = dfs(ops, x, k, 1 + idx, memo);
    let take = ops[idx] + dfs(ops, x, k - 1, idx + x, memo);
    memo[k][idx] = skip.min(take);
    memo[k][idx]
}

// smh O(x) sum is too slow
fn get_min_ops(nums: &[i32], x: usize) -> Vec<i64> {
    use std::collections::BTreeMap;
    let n = nums.len();
    let mut ops = Vec::with_capacity(n - x + 1);
    let [mut small, mut big] = [const { BTreeMap::new() }; 2];
    let [mut small_count, mut big_count] = [0, 0];
    let [mut small_sum, mut big_sum] = [0, 0];
    for (idx, &right) in nums.iter().enumerate() {
        let right = i64::from(right);
        if small.last_key_value().is_none_or(|(&k, _)| k >= right) {
            *small.entry(right).or_insert(0) += 1;
            small_count += 1;
            small_sum += right;
        } else {
            *big.entry(right).or_insert(0) += 1;
            big_count += 1;
            big_sum += right;
        }
        if idx >= x {
            let left = i64::from(nums[idx - x]);
            if let Some(v) = small.get_mut(&left) {
                *v -= 1;
                small_count -= 1;
                small_sum -= left;
                if *v == 0 {
                    small.remove(&left);
                }
            } else if let Some(v) = big.get_mut(&left) {
                *v -= 1;
                big_count -= 1;
                big_sum -= left;
                if *v == 0 {
                    big.remove(&left);
                }
            }
        }
        while small_count < big_count {
            let Some((&k, v)) = big.iter_mut().next() else {
                break;
            };
            *small.entry(k).or_insert(0) += 1;
            small_count += 1;
            small_sum += k;
            *v -= 1;
            big_count -= 1;
            big_sum -= k;
            if *v == 0 {
                big.remove(&k);
            }
        }
        while small_count > 1 + big_count {
            let Some((&k, v)) = small.iter_mut().last() else {
                break;
            };
            *big.entry(k).or_insert(0) += 1;
            big_count += 1;
            big_sum += k;
            *v -= 1;
            small_count -= 1;
            small_sum -= k;
            if *v == 0 {
                small.remove(&k);
            }
        }
        if idx >= x - 1 {
            let med = small.keys().last().unwrap();
            let curr = med * small_count - small_sum + big_sum - med * big_count;
            ops.push(curr);
        }
    }
    ops
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
        assert_eq!(min_operations(&[5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2), 8);
        assert_eq!(min_operations(&[9, -2, -2, -2, 1, 5], 2, 2), 3);
    }

    #[test]
    fn test() {}
}
