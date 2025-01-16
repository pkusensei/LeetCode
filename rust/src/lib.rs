mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_number(cost: &[i32], target: i32) -> String {
    // dfs(cost, target, &mut vec![None; 1 + target as usize])
    let target = target as usize;
    let mut dp = vec![-10000; 1 + target];
    dp[0] = 0;
    for t in 1..=target {
        for i in 0..9 {
            dp[t] = dp[t].max(if t >= cost[i] as usize {
                1 + dp[t - cost[i] as usize]
            } else {
                -10000
            });
        }
    }
    if dp[target] < 0 {
        return "0".to_string();
    }
    let mut temp = target;
    let mut res = "".to_string();
    for i in (0..=8).rev() {
        while temp >= cost[i] as usize && dp[temp] == 1 + dp[temp - cost[i] as usize] {
            res.push(char::from(i as u8 + b'1'));
            temp -= cost[i] as usize;
        }
    }
    res
}

// smh memo my own dfs code leads to wrong answer
// had to take this code
fn dfs(cost: &[i32], target: i32, memo: &mut [Option<String>]) -> String {
    if target == 0 {
        return "".to_string();
    }
    if target < 0 {
        return "0".to_string();
    }
    if let Some(ref v) = memo[target as usize] {
        return v.to_string();
    }
    let mut res = "0".to_string();
    for digit in (1..=9).rev() {
        let mut curr = dfs(cost, target - cost[digit - 1], memo);
        if curr == "0" {
            continue;
        }
        curr = format!("{}{}", digit, curr);
        if res == "0" || curr.len() > res.len() {
            res = curr;
        }
    }
    memo[target as usize] = Some(res.clone());
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(largest_number(&[4, 3, 2, 5, 6, 7, 2, 5, 5], 9), "7772");
        assert_eq!(largest_number(&[7, 6, 5, 5, 5, 6, 8, 7, 8], 12), "85");
        assert_eq!(largest_number(&[2, 4, 6, 2, 4, 6, 4, 4, 4], 5), "0");
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
