mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn maximum_score(nums: &[i32], k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let scores = nums.iter().map(|&v| p_score(v)).collect_vec();
    let mut right_greater = vec![n as i64; n];
    let mut stack = vec![];
    for (idx, &num) in scores.iter().enumerate() {
        while stack.last().is_some_and(|&i| scores[i] < num) {
            let i = stack.pop().unwrap();
            right_greater[i] = idx as i64;
        }
        stack.push(idx);
    }
    let mut left_greater = vec![-1_i64; n];
    stack.clear();
    for (idx, &num) in scores.iter().enumerate().rev() {
        while stack.last().is_some_and(|&i| scores[i] <= num) {
            let i = stack.pop().unwrap();
            left_greater[i] = idx as i64;
        }
        stack.push(idx);
    }
    let ranges = (0..)
        .zip(left_greater.into_iter().zip(right_greater))
        .map(|(i, (left, right))| (i - left) * (right - i))
        .collect_vec();
    let mut k = i64::from(k);
    let mut res = 1;
    for (idx, &num) in nums
        .iter()
        .enumerate()
        .sorted_unstable_by(|a, b| b.1.cmp(a.1))
    {
        if k == 0 {
            break;
        }
        let count = ranges[idx].min(k);
        k -= count;
        res *= mod_pow(num.into(), count, MOD);
        res %= MOD;
    }
    res as i32
}

fn p_score(mut num: i32) -> i32 {
    if num < 2 {
        0
    } else {
        let mut res = 0;
        let sq = num.isqrt();
        for p in 2..=sq {
            let mut temp = true;
            while num % p == 0 {
                num /= p;
                res += i32::from(temp);
                temp = false;
            }
        }
        res += i32::from(num >= 2);
        res
    }
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
        assert_eq!(maximum_score(&[19, 12, 14, 6, 10, 18], 3), 4788);
        assert_eq!(maximum_score(&[8, 3, 9, 3, 8], 2), 81);
    }

    #[test]
    fn test() {}
}
