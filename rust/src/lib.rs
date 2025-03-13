mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_zero_array(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    let mut left = 0;
    let mut right = queries.len();
    if !check(nums, queries, right) {
        return -1;
    }
    while left < right {
        let mid = left + (right - left) / 2;
        if check(nums, queries, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as _
}

fn check(nums: &[i32], queries: &[[i32; 3]], k: usize) -> bool {
    let n = nums.len();
    let mut diff = vec![0; 1 + n];
    for q in queries.iter().take(k) {
        let [left, right] = [0, 1].map(|i| q[i] as usize);
        diff[left] += q[2];
        diff[1 + right] -= q[2];
    }
    let mut prefix = 0;
    for idx in 0..n {
        prefix += diff[idx];
        if prefix < nums[idx] {
            return false;
        }
    }
    true
}

pub fn line_sweep(nums: &[i32], queries: &[[i32; 3]]) -> i32 {
    let n = nums.len();
    let mut prefix = 0;
    let mut res = 0;
    let mut diff = vec![0; 1 + n];
    for (idx, &num) in nums.iter().enumerate() {
        while prefix + diff[idx] < num {
            res += 1;
            if res > queries.len() {
                return -1;
            }
            let [left, right, val] = queries[res - 1];
            if right as usize >= idx {
                diff[(left as usize).max(idx)] += val;
                diff[1 + right as usize] -= val;
            }
        }
        prefix += diff[idx];
    }
    res as _
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
        assert_eq!(
            min_zero_array(&[2, 0, 2], &[[0, 2, 1], [0, 2, 1], [1, 1, 3]]),
            2
        );
        assert_eq!(min_zero_array(&[4, 3, 2, 1], &[[1, 3, 2], [0, 2, 1]]), -1);

        assert_eq!(
            line_sweep(&[2, 0, 2], &[[0, 2, 1], [0, 2, 1], [1, 1, 3]]),
            2
        );
        assert_eq!(line_sweep(&[4, 3, 2, 1], &[[1, 3, 2], [0, 2, 1]]), -1);
    }

    #[test]
    fn test() {}
}
