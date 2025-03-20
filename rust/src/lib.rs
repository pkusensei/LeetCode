mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_subarrays(nums: &[i32], min_k: i32, max_k: i32) -> i64 {
    let [mut max_pos, mut min_pos] = [None, None];
    let mut start = 0;
    let mut res = 0;
    for (idx, &num) in (0..).zip(nums.iter()) {
        if !(min_k..=max_k).contains(&num) {
            max_pos = None;
            min_pos = None;
            start = 1 + idx;
        }
        if num == min_k {
            min_pos = Some(idx);
        }
        if num == max_k {
            max_pos = Some(idx);
        }
        if let Some((a, b)) = min_pos.zip(max_pos) {
            res += a.min(b) - start + 1
        }
    }
    res
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
        assert_eq!(count_subarrays(&[1, 3, 5, 2, 7, 5], 1, 5), 2);
        assert_eq!(count_subarrays(&[1, 1, 1, 1], 1, 1), 10);
    }

    #[test]
    fn test() {}
}
