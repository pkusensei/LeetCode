mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn concatenated_divisibility(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    nums.sort_unstable();
    let mut memo = vec![vec![None; 1 + k as usize]; 1 << n];
    if dfs(&nums, k, 0, 0, &mut memo) {
        build(&nums, &mut memo, k, 0, 0)
    } else {
        vec![]
    }
}

fn dfs(nums: &[i32], k: i32, rem: i32, mask: usize, memo: &mut [Vec<Option<bool>>]) -> bool {
    let n = nums.len();
    if n == mask.count_ones() as usize {
        return rem == 0;
    }
    if let Some(v) = memo[mask][rem as usize] {
        return v;
    }
    let mut res = false;
    for (i, &num) in nums.iter().enumerate() {
        if (mask >> i) & 1 == 0 {
            let width = 1 + num.ilog10();
            let nrem = (rem * 10i32.pow(width) % k + num) % k;
            if dfs(nums, k, nrem, mask | (1 << i), memo) {
                res = true;
                break;
            }
        }
    }
    memo[mask][rem as usize] = Some(res);
    res
}

fn build(nums: &[i32], memo: &mut [Vec<Option<bool>>], k: i32, rem: i32, mask: usize) -> Vec<i32> {
    for (i, &num) in nums.iter().enumerate() {
        if (mask >> i) & 1 == 0 {
            let width = 1 + num.ilog10();
            let nrem = (rem * 10i32.pow(width) % k + num) % k;
            if dfs(nums, k, nrem, mask | (1 << i), memo) {
                let mut res = vec![num];
                res.extend(build(nums, memo, k, nrem, mask | (1 << i)));
                return res;
            }
        }
    }
    vec![]
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
        assert_eq!(concatenated_divisibility(vec![3, 12, 45], 5), [3, 12, 45]);
        assert_eq!(concatenated_divisibility(vec![10, 5], 10), [5, 10]);
        assert_eq!(concatenated_divisibility(vec![1, 2, 3], 5), []);
    }

    #[test]
    fn test() {}
}
