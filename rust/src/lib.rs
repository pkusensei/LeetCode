mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    let mt = mountain_height as u128;
    let mut left = 1;
    let mut right = 10_u128.pow(17);
    while left < right {
        let mid = left + (right - left) / 2;
        if check(mt, &worker_times, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left as i64
}

fn check(mut mt: u128, times: &[i32], mid: u128) -> bool {
    for &t in times {
        let Some(v) = mt.checked_sub(reduce(t as _, mid)) else {
            return true;
        };
        mt = v;
    }
    mt == 0
}

fn reduce(t: u128, max: u128) -> u128 {
    let mut left = 0;
    let mut right = max;
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if (1 + mid) * mid / 2 * t > max {
            right = mid - 1;
        } else {
            left = mid;
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
        assert_eq!(min_number_of_seconds(4, vec![2, 1, 1]), 3);
        assert_eq!(min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
        assert_eq!(min_number_of_seconds(5, vec![1]), 15);
    }

    #[test]
    fn test() {
        assert_eq!(min_number_of_seconds(1, vec![5]), 5);
    }
}
