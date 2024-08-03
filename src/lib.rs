pub fn min_distance(word1: &str, word2: &str) -> i32 {
    if word1.is_empty() {
        return word2.len() as _;
    }
    if word2.is_empty() {
        return word1.len() as _;
    }
    if word1 == word2 {
        return 0;
    }
    let row = word1.len();
    let col = word2.len();
    let mut dp = vec![vec![0; col + 1]; row + 1];

    // First row and col means
    // The dist between string of length x and empty string is x
    for (j, v) in dp[0].iter_mut().enumerate() {
        *v = j;
    }
    for (i, v) in dp.iter_mut().enumerate() {
        v[0] = i
    }
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    for i in 1..=row {
        for j in 1..=col {
            if w1[i - 1] == w2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // replacing i-1 with j-1
                // delete i-1 from word1
                // insert j-1 into word1
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }
    dp[row][col] as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_distance("horse", "ros"), 3);
        debug_assert_eq!(min_distance("intention", "execution"), 5);
    }

    #[test]
    fn test() {}
}
