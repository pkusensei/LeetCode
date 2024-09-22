mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_substring_in_wrapround_string(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut dp = [0; 26];
    let mut curr_max_length = 1;
    for (i, &b) in s.iter().enumerate() {
        let idx = usize::from(b - b'a');
        if i > 0 && (s[i - 1] + 1 == b || s[i - 1] - 25 == b) {
            curr_max_length += 1;
        } else {
            curr_max_length = 1;
        }
        dp[idx] = dp[idx].max(curr_max_length);
    }
    dp.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_substring_in_wrapround_string("a"), 1);
        debug_assert_eq!(find_substring_in_wrapround_string("cac"), 2);
        debug_assert_eq!(find_substring_in_wrapround_string("zab"), 6);
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
