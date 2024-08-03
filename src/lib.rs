pub fn climb_stairs(n: i32) -> i32 {
    if n < 2 {
        return 1;
    }
    let mut dp = vec![0; n as usize];
    dp[0] = 1;
    dp[1] = 2;
    for i in 2..(n as usize) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(climb_stairs(2), 2);
        debug_assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test() {}
}
