mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_excellent_pairs(mut nums: Vec<i32>, k: i32) -> i64 {
    nums.sort_unstable_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));
    nums.dedup();
    let n = nums.len();
    let mut res = 0;
    for &a in nums.iter() {
        let i = nums.partition_point(|&v| v.count_ones() + a.count_ones() < k as u32);
        res += n - i;
    }
    res as i64
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
        assert_eq!(count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
        assert_eq!(count_excellent_pairs(vec![5, 1, 1], 10), 0);
    }

    #[test]
    fn test() {}
}
