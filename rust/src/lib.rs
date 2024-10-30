mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn score_of_parentheses(s: &str) -> i32 {
    dfs(s)
}

fn dfs(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    if s == "()" {
        return 1;
    }
    let mut open = 0;
    for (idx, b) in s.bytes().enumerate() {
        match b {
            b'(' => open += 1,
            _ => open -= 1,
        }
        if open == 0 {
            if idx == s.len() - 1 {
                return 2 * dfs(&s[1..s.len() - 1]);
            } else {
                return dfs(&s[..=idx]) + dfs(&s[1 + idx..]);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(score_of_parentheses("()"), 1);
        debug_assert_eq!(score_of_parentheses("(())"), 2);
        debug_assert_eq!(score_of_parentheses("()()"), 2);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
