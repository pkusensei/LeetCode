mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_strength(strength: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = strength.len();
    let mut right_smaller = vec![n; n];
    let mut stack = vec![];
    for (idx, &num) in strength.iter().enumerate() {
        while stack.last().is_some_and(|&v| strength[v] > num) {
            let top = stack.pop().unwrap();
            right_smaller[top] = idx;
        }
        stack.push(idx);
    }
    let mut left_smaller = vec![-1; n];
    stack.clear();
    for (idx, &num) in strength.iter().enumerate().rev() {
        while stack.last().is_some_and(|&v| strength[v] >= num) {
            let top = stack.pop().unwrap();
            left_smaller[top] = idx as i64;
        }
        stack.push(idx);
    }
    let presum = strength
        .iter()
        .fold(Vec::with_capacity(n), |mut acc, &num| {
            acc.push((i64::from(num) + acc.last().unwrap_or(&0)) % MOD);
            acc
        });
    let presum_presum = presum.iter().fold(vec![0], |mut acc, &num| {
        acc.push((num + acc.last().unwrap_or(&0)) % MOD);
        acc
    });
    let mut res = 0;
    for (idx, &num) in presum_presum[..n].iter().enumerate() {
        let left = left_smaller[idx];
        let right = right_smaller[idx];
        let left_sum = num - presum_presum[left.max(0) as usize];
        let right_sum = presum_presum[right] - num;
        res += i64::from(strength[idx])
            * (right_sum * (idx as i64 - left) % MOD - (left_sum * (right - idx) as i64) % MOD);
        res = res.rem_euclid(MOD);
    }
    res as i32
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
        assert_eq!(total_strength(&[1, 3, 1, 2]), 44);
        assert_eq!(total_strength(&[5, 4, 6]), 213);
    }

    #[test]
    fn test() {}
}
