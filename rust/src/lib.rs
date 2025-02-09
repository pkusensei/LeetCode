mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_string(s: &str) -> bool {
    dfs(s, 0, None)
}

fn dfs(s: &str, start: usize, prev: Option<u64>) -> bool {
    let n = s.len();
    if start >= n {
        return prev.is_some();
    }
    let mut end = start;
    while s.bytes().nth(end).is_some_and(|b| b == b'0') {
        end += 1;
    }
    let mut res = false;
    for i in end..=n {
        let Ok(num) = s[start..i].parse() else {
            continue;
        };
        if let Some(prev) = prev {
            res |= prev == 1 + num && dfs(s, i, Some(num));
        } else {
            res |= i <= n - 1 && dfs(s, i, Some(num))
        }
        if res {
            break;
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
        assert!(split_string("050043"));
        assert!(!split_string("1234"));
        assert!(!split_string("9080701"));
    }

    #[test]
    fn test() {
        assert!(split_string("10"));
    }
}
