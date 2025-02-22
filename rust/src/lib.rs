mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_subsequence(s: &str, k: i32, letter: char, repetition: i32) -> String {
    let (n, k) = (s.len(), k as usize);
    let mut rep = repetition as usize;
    let count = s.chars().filter(|&c| c == letter).count();
    let mut extra = count - rep; // count of potential pops of letter
    let mut del = n - k; // count of potential pops in total
    let mut stack = vec![];
    for ch in s.chars() {
        while stack.last().is_some_and(|&v| v > ch && del > 0) {
            if stack.last().is_some_and(|&v| v == letter && extra == 0) {
                break;
            }
            extra -= usize::from(stack.last().is_some_and(|&v| v == letter));
            del -= 1;
            stack.pop();
        }
        stack.push(ch);
    }
    let mut res = String::with_capacity(k);
    for ch in stack {
        if ch != letter && res.len() + rep >= k {
            continue; // enough candidates left => skip this letter
        }
        res.push(ch);
        rep = rep.saturating_sub(usize::from(ch == letter));
        if res.len() >= k {
            break;
        }
    }
    res
}

// TLE
fn dfs(s: &[u8], letter: u8, k: i32, rep: i32, curr: &mut Vec<u8>, res: &mut Vec<u8>) {
    match s {
        [] => {
            if k == 0 && rep <= 0 && (res.is_empty() || curr < res) {
                *res = curr.clone()
            }
        }
        [head, tail @ ..] => {
            dfs(tail, letter, k, rep, curr, res);
            curr.push(*head);
            let is_match = i32::from(*head == letter);
            dfs(tail, letter, k - 1, rep - is_match, curr, res);
            curr.pop();
        }
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
        assert_eq!(smallest_subsequence("leet", 3, 'e', 1), "eet");
        assert_eq!(smallest_subsequence("leetcode", 4, 'e', 2), "ecde");
        assert_eq!(smallest_subsequence("bb", 2, 'b', 2), "bb");
    }

    #[test]
    fn test() {}
}
