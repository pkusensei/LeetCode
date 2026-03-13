mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut left = 1;
    let mut right = i64::MAX >> 2;
    while left < right {
        let mid = left + (right - left) / 2;
        let mut curr = 0;
        for &t in worker_times.iter() {
            curr += find_height(t, mid);
            if curr >= mountain_height {
                break;
            }
        }
        if curr >= mountain_height {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
}

fn find_height(time: i32, max: i64) -> i32 {
    let mut left = 0;
    let mut right = 100_001;
    while left < right {
        let mid = left + (1 + right - left) / 2;
        let curr = i64::from(time) * (1 + mid) * mid / 2;
        if curr > max {
            right = mid - 1
        } else {
            left = mid;
        }
    }
    left as i32
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
        assert_eq!(min_number_of_seconds(4, &[2, 1, 1]), 3);
    }

    #[test]
    fn test() {
        assert_eq!(min_number_of_seconds(6, &[8]), 168);
        assert_eq!(min_number_of_seconds(1, &[5]), 5);
    }
}
