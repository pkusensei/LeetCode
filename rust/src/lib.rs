mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_beautiful_substrings(s: &str) -> i32 {
    let n = s.len();
    dfs(s.as_bytes(), 0, &mut vec![None; n]).unwrap_or(-1)
}

fn dfs(s: &[u8], idx: usize, memo: &mut [Option<i32>]) -> Option<i32> {
    let n = s.len();
    if idx >= n {
        return Some(0);
    }
    if s[idx] == b'0' {
        return None;
    }
    if let Some(v) = memo[idx] {
        return if v == -1 { None } else { Some(v) };
    }
    let mut curr = 0;
    let mut res = i32::MAX;
    for i in idx..n {
        curr = curr * 2 + i32::from(s[i] - b'0');
        if check(curr) {
            if let Some(v) = dfs(s, 1 + i, memo) {
                res = res.min(v);
            }
        }
    }
    if res == i32::MAX {
        memo[idx] = Some(-1);
        None
    } else {
        memo[idx] = Some(1 + res);
        Some(1 + res)
    }
}

const fn check(mut num: i32) -> bool {
    while num > 1 && num % 5 == 0 {
        num /= 5
    }
    num == 1
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
        assert_eq!(minimum_beautiful_substrings("1011"), 2);
        assert_eq!(minimum_beautiful_substrings("111"), 3);
        assert_eq!(minimum_beautiful_substrings("0"), -1);
    }

    #[test]
    fn test() {}
}
