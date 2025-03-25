mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_power(stations: &[i32], r: i32, k: i32) -> i64 {
    let r = r as usize;
    let k = i64::from(k);
    let n = stations.len();
    let mut powers = vec![0; 1 + n];
    let mut sum = 0;
    for (i, &v) in stations.iter().enumerate() {
        let v = i64::from(v);
        let a = i.saturating_sub(r);
        let b = (i + r + 1).min(n);
        powers[a] += v;
        powers[b] -= v;
        sum += v;
    }
    powers.pop();
    let mut curr = 0;
    let mut left = i64::MAX;
    for v in powers.iter_mut() {
        curr += *v;
        *v = curr;
        left = left.min(curr);
    }
    let mut right = sum + k;
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        if check(&powers, r, k, mid) {
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
            let diff = mid - curr;
            k -= diff;
            if k < 0 {
                return false;
            }
            let right = (left + r + r).min(n - 1);
            prefix[left] += diff;
            prefix[right + 1] -= diff;
        }
    }
    k >= 0
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
        assert_eq!(max_power(&[1, 2, 4, 5, 0], 1, 2), 5);
        assert_eq!(max_power(&[4, 4, 4, 4], 0, 3), 4);
    }

    #[test]
    fn test() {}
}
