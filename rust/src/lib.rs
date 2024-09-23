mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_extra_char(s: &str, dictionary: &[&str]) -> i32 {
    // solve(s, dictionary, &mut HashMap::new()) as _
    let n = s.len();
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = dp[i - 1] + 1;
        for w in dictionary.iter() {
            if s[..i].ends_with(w) {
                dp[i] = dp[i].min(dp[i - w.len()]);
            }
        }
    }
    dp[n]
}

fn solve<'a>(s: &'a str, dictionary: &[&str], seen: &mut HashMap<&'a str, usize>) -> usize {
    if s.is_empty() {
        return 0;
    }
    if let Some(&v) = seen.get(s) {
        return v;
    }
    let mut res = 1 + solve(&s[1..], dictionary, seen);
    for w in dictionary {
        if let Some(r) = s.strip_prefix(w) {
            res = res.min(solve(r, dictionary, seen));
        }
    }
    seen.insert(s, res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            min_extra_char("leetscode", &["leet", "code", "leetcode"]),
            1
        );
        debug_assert_eq!(min_extra_char("sayhelloworld", &["hello", "world"]), 3);
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
