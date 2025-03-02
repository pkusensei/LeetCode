mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_run_time(n: i32, batteries: &mut [i32]) -> i64 {
    use std::cmp::Reverse;

    let n = n as usize;
    batteries.sort_unstable_by_key(|&v| Reverse(v));
    let mut live: Vec<_> = batteries[..n].iter().copied().map(i64::from).collect();
    live.reverse();
    let mut extra: i64 = batteries[n..].iter().copied().map(i64::from).sum();
    for idx in 0..n - 1 {
        let curr = (1 + idx as i64) * (live[1 + idx] - live[idx]);
        if extra < curr {
            return live[idx] + extra / (1 + idx as i64);
        }
        extra -= curr;
    }
    live[n - 1] + extra / n as i64
}

pub fn with_binary_search(n: i32, batteries: &[i32]) -> i64 {
    let n = i64::from(n);
    let sum: i64 = batteries.iter().copied().map(i64::from).sum();
    let mut left = 1;
    let mut right = sum / n;
    while left < right {
        let mid = right - (right - left) / 2;
        let extra: i64 = batteries.iter().map(|&v| i64::from(v).min(mid)).sum();
        if extra >= n * mid {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
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
        assert_eq!(max_run_time(2, &mut [3, 3, 3]), 4);
        assert_eq!(max_run_time(2, &mut [1, 1, 1, 1]), 2);

        assert_eq!(with_binary_search(2, &[3, 3, 3]), 4);
        assert_eq!(with_binary_search(2, &[1, 1, 1, 1]), 2);
    }

    #[test]
    fn test() {}
}
