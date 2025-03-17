mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_special_numbers(n: i32) -> i32 {
    (1..=9).map(|d| dfs(i64::from(n), d, 1 << d)).sum()
}

fn dfs(n: i64, curr: i64, mask: i32) -> i32 {
    if curr > n {
        return 0;
    }
    let mut res = 1;
    for d in 0..=9 {
        if (mask >> d) & 1 == 0 {
            res += dfs(n, 10 * curr + d, mask | (1 << d))
        }
    }
    res
}

pub fn with_perm(n: i32) -> i32 {
    fn perm(n: i32, k: i32) -> i32 {
        (n - k + 1..=n).product()
    }

    let mut n = 1 + n;
    let mut digits: Vec<_> = std::iter::from_fn(move || {
        if n > 0 {
            let d = n % 10;
            n /= 10;
            Some(d)
        } else {
            None
        }
    })
    .collect();
    digits.reverse();
    let len = digits.len();
    let mut res = (1..len).map(|v| 9 * perm(9, v as i32 - 1)).sum();
    let mut mask = 0;
    for (idx, &digit) in digits.iter().enumerate() {
        let start = i32::from(idx == 0);
        for d in start..digit {
            if (mask >> d) & 1 == 0 {
                res += perm(9 - idx as i32, (len - idx) as i32);
            }
        }
        if (mask >> digit) & 1 == 1 {
            break;
        }
        mask |= 1 << digit;
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
        assert_eq!(count_special_numbers(20), 19);
        assert_eq!(count_special_numbers(5), 5);
        assert_eq!(count_special_numbers(135), 110);
    }

    #[test]
    fn test() {
        assert_eq!(count_special_numbers(820486701), 4968690);
    }
}
