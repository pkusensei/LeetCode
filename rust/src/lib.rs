mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn good_indices(nums: &[i32], k: i32) -> Vec<i32> {
    let mut dec = vec![0];
    let mut curr = 0;
    for w in nums.windows(2) {
        if w[0] >= w[1] {
            curr += 1;
        } else {
            curr = 0;
        }
        dec.push(curr);
    }
    let mut inc = vec![];
    curr = 0;
    for w in nums.windows(2).rev() {
        if w[0] <= w[1] {
            curr += 1
        } else {
            curr = 0
        }
        inc.push(curr);
    }
    inc.reverse();
    inc.push(0);
    let n = nums.len();
    let mut res = vec![];
    if n <= 2 * k as usize {
        return res;
    }
    for i in k as usize..n - k as usize {
        if dec[i - 1] >= k - 1 && inc[i + 1] >= k - 1 {
            res.push(i as i32);
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
        assert_eq!(good_indices(&[2, 1, 1, 1, 3, 4, 1], 2), [2, 3]);
    }

    #[test]
    fn test() {}
}
