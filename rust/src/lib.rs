mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_sum(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut arr = Vec::with_capacity(2 * n + 1);
    let mut prefix = Vec::with_capacity(1 + n);
    prefix.push(0);
    let mut res = 0;
    for &num in nums.iter() {
        res = res.max(i64::from(num)); // single element
        arr.extend([-1, num]);
        prefix.push(i64::from(num) + prefix.last().unwrap_or(&0));
    }
    arr.push(-1); // [-1, 10, -1, 10, -1]
    for (i, val) in manacher_odd(arr).into_iter().enumerate() {
        if val <= 1 {
            continue;
        }
        let left = (i - val) / 2;
        let right = (i + val) / 2;
        let curr = prefix[right] - prefix[left];
        res = res.max(curr);
    }
    res
}

fn manacher_odd(arr: Vec<i32>) -> Vec<usize> {
    let n = arr.len();
    let mut palin = vec![0; n];
    let mut center = 0;
    let mut right = 0;
    for i in 0..n {
        if i < right {
            palin[i] = (right - i).min(palin[2 * center - i]);
        }
        while i >= 1 + palin[i]
            && i + 1 + palin[i] < n
            && arr[i - 1 - palin[i]] == arr[i + 1 + palin[i]]
        {
            palin[i] += 1;
        }
        if i + palin[i] > right {
            center = i;
            right = i + palin[i];
        }
    }
    palin
}

const M: i64 = 1_000_000_007;
const BASE: i64 = 31;

pub fn with_rolling_hash(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut prefix_sum = vec![0; 1 + n];
    let mut prefix_hash = vec![0; 1 + n];
    let mut rev_hash = vec![0; 1 + n];
    let mut pows = vec![1; 1 + n];
    for (i, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        prefix_sum[1 + i] = prefix_sum[i] + num;
        prefix_hash[1 + i] = (prefix_hash[i] * BASE % M + num) % M;
        pows[1 + i] = pows[i] * BASE % M;
    }
    for (i, &num) in nums.iter().rev().enumerate() {
        let num = i64::from(num);
        rev_hash[1 + i] = (rev_hash[i] * BASE % M + num) % M;
    }
    let mut res = 0;
    for i in 0..n {
        // odd length
        let mut low = 0;
        let mut high = i.min(n - i - 1);
        while low < high {
            let mid = low + (1 + high - low) / 2;
            if check(&prefix_hash, &rev_hash, &pows, i - mid, i + mid) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        let left = i - low;
        let right = i + low;
        res = res.max(prefix_sum[1 + right] - prefix_sum[left]);
        // even length
        low = 0;
        high = i.min(n - i);
        while low < high {
            let mid = low + (1 + high - low) / 2;
            if check(&prefix_hash, &rev_hash, &pows, i - mid, i + mid - 1) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        if low > 0 {
            let left = i - low;
            let right = i + low - 1;
            res = res.max(prefix_sum[1 + right] - prefix_sum[left]);
        }
    }
    res
}

fn check(prefix_hash: &[i64], rev_hash: &[i64], pows: &[i64], left: usize, right: usize) -> bool {
    let n = prefix_hash.len() - 1;
    if left > right {
        return true;
    }
    let fore =
        (prefix_hash[1 + right] - prefix_hash[left] * pows[right + 1 - left] % M).rem_euclid(M);
    let rev_left = n - 1 - right;
    let rev_right = n - 1 - left;
    let back =
        (rev_hash[1 + rev_right] - rev_hash[rev_left] * pows[1 + right - left] % M).rem_euclid(M);
    fore == back
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
        assert_eq!(get_sum(&[10, 10]), 20);
        assert_eq!(get_sum(&[1, 2, 3, 2, 1, 5, 6]), 9);

        assert_eq!(with_rolling_hash(&[10, 10]), 20);
        assert_eq!(with_rolling_hash(&[1, 2, 3, 2, 1, 5, 6]), 9);
    }

    #[test]
    fn test() {}
}
