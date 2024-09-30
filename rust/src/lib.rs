mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: i32 = 1_000_000_007;

pub fn check_record(n: i32) -> i32 {
    bottom_up(n)
    // let n = n as usize;
    // // dp[absent][late][1..=n]
    // let mut dp = vec![vec![vec![-1; 1 + n]; 3]; 2];
    // top_down(&mut dp, n, 0, 0)
}

// stack overflow
fn top_down(dp: &mut [Vec<Vec<i32>>], n: usize, abs: usize, late: usize) -> i32 {
    if abs >= 2 || late >= 3 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    if dp[abs][late][n] > -1 {
        return dp[abs][late][n];
    }
    // 'P' => clears late records, but keeps absent
    let mut res = top_down(dp, n - 1, abs, 0) % MOD;
    // 'A' => clears late records, but keeps absent
    res = (res + top_down(dp, n - 1, abs + 1, 0)) % MOD;
    // 'L' => consecutive late
    res = (res + top_down(dp, n - 1, abs, late + 1)) % MOD;
    dp[abs][late][n] = res;
    res
}

fn bottom_up(n: i32) -> i32 {
    // dp[abs][late]
    let mut dp_curr = [[0; 3]; 2];
    let mut dp_next = dp_curr;
    dp_curr[0][0] = 1;
    for _ in 0..n {
        for abs in 0..2 {
            for lat in 0..3 {
                // 'P'
                dp_next[abs][0] = (dp_next[abs][0] + dp_curr[abs][lat]) % MOD;
                if abs == 0 {
                    // 'A'
                    dp_next[1][0] = (dp_next[1][0] + dp_curr[0][lat]) % MOD;
                }
                if lat < 2 {
                    // 'L'
                    dp_next[abs][lat + 1] = (dp_next[abs][lat + 1] + dp_curr[abs][lat]) % MOD;
                }
            }
        }
        dp_curr = dp_next;
        dp_next = Default::default();
    }
    dp_curr
        .into_iter()
        .flat_map(|r| r.into_iter())
        .fold(0, |acc, n| (acc + n) % MOD)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(check_record(2), 8);
        debug_assert_eq!(check_record(1), 3);
        debug_assert_eq!(check_record(10101), 183236316);
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
