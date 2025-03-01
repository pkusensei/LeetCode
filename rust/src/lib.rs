mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_increasing(arr: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let mut nums = vec![vec![]; k];
    for (i, &num) in arr.iter().enumerate() {
        nums[i % k].push(num);
    }
    nums.iter().map(|v| v.len() as i32 - find_lis(v)).sum()
}

fn find_lis(nums: &[i32]) -> i32 {
    let mut res = vec![];
    for &num in nums {
        let i = res.partition_point(|&v| v <= num);
        if i == res.len() {
            res.push(num);
        } else {
            res[i] = num;
        }
    }
    res.len() as _
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
        assert_eq!(k_increasing(&[5, 4, 3, 2, 1], 1), 4);
        assert_eq!(k_increasing(&[4, 1, 5, 2, 6, 2], 2), 0);
        assert_eq!(k_increasing(&[4, 1, 5, 2, 6, 2], 3), 2);
    }

    #[test]
    fn test() {}
}
