pub fn is_interleave(s1: &str, s2: &str, s3: &str) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
    dp[0][0] = true;
    for (i1, (b1, b3)) in s1.bytes().zip(s3.bytes()).enumerate() {
        dp[i1 + 1][0] = dp[i1][0] && b1 == b3;
    }
    for (i2, (b2, b3)) in s2.bytes().zip(s3.bytes()).enumerate() {
        dp[0][i2 + 1] = dp[0][i2] && b2 == b3;
    }

    for (i1, b1) in s1.bytes().enumerate().map(|(i, b)| (i + 1, b)) {
        for (i2, b2) in s2.bytes().enumerate().map(|(i, b)| (i + 1, b)) {
            // start with s3[1] because
            // dp[0][..] and dp[..][0] have compared s1[0] and s2[0] against s3[0]
            // s3[0] option is already exhausted
            let b3 = s3.as_bytes()[i1 + i2 - 1];
            dp[i1][i2] = (dp[i1 - 1][i2] && b1 == b3) || (dp[i1][i2 - 1] && b2 == b3);
        }
    }
    dp[s1.len()][s2.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(is_interleave("aabcc", "dbbca", "aadbbcbcac"), true);
        debug_assert_eq!(is_interleave("aabcc", "dbbca", "aadbbbaccc"), false);
        debug_assert_eq!(is_interleave("", "", ""), true);
    }

    #[test]
    fn test() {}
}
