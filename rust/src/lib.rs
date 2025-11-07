mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_power(stations: &[i32], r: i32, k: i32) -> i64 {
    let n = stations.len();
    let mut line = vec![0; 1 + n];
    let r = r as usize;
    let mut sum = 0;
    for (i, &v) in stations.iter().enumerate() {
        let v = i64::from(v);
        sum += v;
        let left = i.saturating_sub(r);
        line[left] += v;
        let right = (i + r + 1).min(n);
        line[right] -= v;
    }
    let mut curr = 0;
    let mut left = i64::MAX;
    for v in line.iter_mut() {
        curr += *v;
        *v = curr;
        left = left.min(curr);
    }
    let k = i64::from(k);
    let mut right = sum + k;
    line.pop();
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if check(&line, r, k, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn check(nums: &[i64], r: usize, mut k: i64, mid: i64) -> bool {
    let n = nums.len();
    let mut prefix = vec![0; 1 + n];
    for (left, &num) in nums.iter().enumerate() {
        if left > 0 {
            prefix[left] += prefix[left - 1];
        }
        let curr = num + prefix[left];
        if curr < mid {
            let delta = mid - curr;
            k -= delta;
            if k < 0 {
                return false;
            }
            prefix[left] += delta;
            let right = (left + 2 * r + 1).min(n);
            prefix[right] -= delta;
        }
    }
    true
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
        assert_eq!(max_power(&[1, 2, 4, 5, 0], 1, 2), 5);
    }

    #[test]
    fn test() {}
}
