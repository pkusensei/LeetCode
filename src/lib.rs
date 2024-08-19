mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn min_steps(n: i32) -> i32 {
    fn with_dp(n: i32) -> i32 {
        let mut dp = vec![1000; n as usize + 1];
        dp[1] = 0;
        for i in 2..n as usize {
            for j in 1..=i / 2 {
                if i % j == 0 {
                    dp[i] = dp[i].min(dp[j] + i / j)
                }
            }
        }
        dp[n as usize] as _
    }

    let mut prime = 2;
    let mut res = 0;
    let mut n = n;
    while n > 1 {
        while n % prime == 0 {
            res += prime;
            n /= prime
        }
        prime += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_steps(3), 3);
        debug_assert_eq!(min_steps(1), 0);
    }

    #[test]
    fn test() {}
}
