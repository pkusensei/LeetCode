mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_subarray_beauty(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let mut res = vec![];
    let mut freq = [0; 50];
    'outer: for (idx, &num) in nums.iter().enumerate() {
        if num < 0 {
            freq[(num + 50) as usize] += 1;
        }
        if idx >= k && nums[idx - k] < 0 {
            freq[(nums[idx - k] + 50) as usize] -= 1;
        }
        if idx >= k - 1 {
            let mut count = 0;
            for (val, f) in (-50..).zip(freq.iter()) {
                count += f;
                if count >= x {
                    res.push(val);
                    continue 'outer;
                }
            }
            res.push(0);
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
        assert_eq!(get_subarray_beauty(&[1, -1, -3, -2, 3], 3, 2), [-1, -2, -2]);
        assert_eq!(
            get_subarray_beauty(&[-1, -2, -3, -4, -5], 2, 2),
            [-1, -2, -3, -4]
        );
        assert_eq!(
            get_subarray_beauty(&[-3, 1, 2, -3, 0, -3], 2, 1),
            [-3, 0, -3, -3, -3]
        );
    }

    #[test]
    fn test() {}
}
