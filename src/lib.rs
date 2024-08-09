pub fn num_distinct(s: &str, t: &str) -> i32 {
    let (s1, s2) = (s.len(), t.len());
    if s1 < s2 {
        return 0;
    }
    let mut dp = vec![vec![0; s2 + 1]; s1 + 1];
    for i1 in 0..=s1 {
        // form emtry string from s
        dp[i1][0] = 1;
    }
    for (i1, b1) in s.bytes().enumerate().map(|(i, b)| (i + 1, b)) {
        for (i2, b2) in t.bytes().enumerate().map(|(i, b)| (i + 1, b)) {
            // to form t[..i2-1] from s[..i1-1] <== dp[i1][i2]
            if b1 == b2 {
                // include s[i1-1]
                dp[i1][i2] = dp[i1 - 1][i2 - 1] + dp[i1 - 1][i2];
            } else {
                // use only s[..i1-2]
                dp[i1][i2] = dp[i1 - 1][i2];
            }
        }
    }
    dp[s1][s2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_distinct("rabbbit", "rabbit"), 3);
        debug_assert_eq!(num_distinct("babgbag", "bag"), 5);
    }

    #[test]
    fn test() {}
}
