mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_form(strs: &[&str], m: i32, n: i32) -> i32 {
    // let strs: Vec<_> = strs.iter().map(|s| count_zero_one(s)).collect();
    // dfs(&strs, m, n, &mut HashMap::new())
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; 1 + m]; 1 + n];
    for [zero, one] in strs.iter().map(|s| count_zero_one(s).map(|n| n as usize)) {
        for ones in (one..=n).rev() {
            for zeros in (zero..=m).rev() {
                dp[ones][zeros] = dp[ones][zeros].max(1 + dp[ones - one][zeros - zero]);
            }
        }
    }
    dp[n][m]
}

fn dfs(
    strs: &[[i32; 2]],
    zeros: i32,
    ones: i32,
    seen: &mut HashMap<(usize, i32, i32), i32>,
) -> i32 {
    if let Some(&v) = seen.get(&(strs.len(), zeros, ones)) {
        return v;
    }
    match strs {
        [] => 0,
        [head, tail @ ..] => {
            let mut res = dfs(tail, zeros, ones, seen);
            if zeros - head[0] >= 0 && ones - head[1] >= 0 {
                res = res.max(1 + dfs(tail, zeros - head[0], ones - head[1], seen))
            }
            seen.insert((strs.len(), zeros, ones), res);
            res
        }
    }
}

fn count_zero_one(s: &str) -> [i32; 2] {
    s.bytes().fold([0, 0], |mut acc, b| {
        if b == b'0' {
            acc[0] += 1
        } else {
            acc[1] += 1
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_max_form(&["10", "0001", "111001", "1", "0"], 5, 3), 4);
        debug_assert_eq!(find_max_form(&["10", "0", "1"], 1, 1), 2);
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
