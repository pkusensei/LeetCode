pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![1; n as usize];
    for _ in 1..m {
        for j in 1..n {
            dp[j as usize] += dp[j as usize - 1];
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(unique_paths(3, 7), 28);
        debug_assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn test() {}
}
