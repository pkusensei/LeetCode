mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_suffix(nums: Vec<i32>, target: Vec<i32>) -> Vec<i32> {
    let [n1, n2] = [&nums, &target].map(|v| v.len());
    let mut freq = vec![0; 100_001];
    let mut diff = 0;
    for &num in nums.iter() {
        diff = update(&mut freq, num, 1, diff);
    }
    for &num in target[..n1].iter() {
        diff = update(&mut freq, num, -1, diff);
    }
    let lcp = build_lcp(&nums, &target);
    let last_bad = build_last_bad(&target);
    let len = 1 + n2 - n1;
    let mut res = vec![0; len];
    for k in 0..len {
        if diff == 0 {
            let mut lower = 0;
            if n1 > 1 {
                if let Some(last) = last_bad[k + n1 - 1]
                    && last >= k
                {
                    lower = 1 + (last - k) as i32;
                }
            }
            let upper = lcp[k].min(n1 - 1) as i32;
            res[k] = (1 + upper - lower).max(0);
        }
        if k + n1 < n2 {
            diff = update(&mut freq, target[k], 1, diff);
            diff = update(&mut freq, target[k + n1], -1, diff);
        }
    }
    res
}

fn update(freq: &mut [i32], num: i32, val: i32, mut diff: i32) -> i32 {
    let num = num as usize;
    diff += i32::from(freq[num] == 0);
    freq[num] += val;
    diff -= i32::from(freq[num] == 0);
    diff
}

fn build_lcp(nums: &[i32], target: &[i32]) -> Vec<usize> {
    use itertools::{Itertools, chain};
    let combined = chain!(nums, std::iter::once(&0), target)
        .copied()
        .collect_vec();
    let z = z_func(&combined);
    let [n1, n2] = [nums, target].map(|v| v.len());
    let _n = 1 + n2 - n1;
    (0.._n).map(|k| n1.min(z[n1 + k + 1])).collect()
}

fn z_func(arr: &[i32]) -> Vec<usize> {
    let n = arr.len();
    let mut z = vec![0; n];
    let [mut left, mut right] = [0, 0];
    for i in 1..n {
        if i <= right {
            z[i] = (1 + right - i).min(z[i - left]);
        }
        while i + z[i] < n && arr[z[i]] == arr[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > right {
            left = i;
            right = i + z[i] - 1;
        }
    }
    z
}

fn build_last_bad(target: &[i32]) -> Vec<Option<usize>> {
    let n = target.len();
    let mut last = vec![None; n];
    for i in 1..n {
        last[i] = last[i - 1];
        if target[i - 1] < target[i] {
            last[i] = Some(i - 1);
        }
    }
    last
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
