mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
    let (n1, n2) = (num1.len(), num2.len());
    let mut v = num1.into_bytes();
    for b in v.iter_mut().rev() {
        if *b == b'0' {
            *b = b'9';
        } else {
            *b -= 1;
            break;
        }
    }
    let a = dfs(
        &v,
        min_sum,
        max_sum,
        0,
        0,
        true,
        &mut vec![vec![-1; 450]; n1],
    );
    let b = dfs(
        num2.as_bytes(),
        min_sum,
        max_sum,
        0,
        0,
        true,
        &mut vec![vec![-1; 450]; n2],
    );
    (b - a).rem_euclid(MOD)
}

const MOD: i32 = 1_000_000_007;

fn dfs(
    max: &[u8],
    min_sum: i32,
    max_sum: i32,
    idx: usize,
    sum: i32,
    limit: bool,
    memo: &mut [Vec<i32>],
) -> i32 {
    if idx >= max.len() {
        return i32::from((min_sum..=max_sum).contains(&sum));
    }
    if !limit && memo[idx][sum as usize] > -1 {
        return memo[idx][sum as usize];
    }
    let mut res = 0;
    let upper = if limit { i32::from(max[idx] - b'0') } else { 9 };
    for d in 0..=upper {
        res += dfs(
            max,
            min_sum,
            max_sum,
            1 + idx,
            sum + d,
            limit && d == upper,
            memo,
        );
        res %= MOD;
    }
    if !limit {
        memo[idx][sum as usize] = res;
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
        assert_eq!(count("1".into(), "12".into(), 1, 8), 11);
        assert_eq!(count("1".into(), "5".into(), 1, 5), 5);
    }

    #[test]
    fn test() {
        assert_eq!(
            count("4179205230".into(), "7748704426".into(), 8, 46),
            883045899
        );
    }
}
