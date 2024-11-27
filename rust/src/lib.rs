mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_common_supersequence(str1: &str, str2: &str) -> String {
    let [s1, s2] = [str1, str2].map(|s| s.as_bytes());
    let [n1, n2] = [s1, s2].map(|s| s.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for (i1, &b1) in s1.iter().enumerate() {
        for (i2, &b2) in s2.iter().enumerate() {
            if b1 == b2 {
                dp[i1 + 1][i2 + 1] = 1 + dp[i1][i2];
            } else {
                dp[i1 + 1][i2 + 1] = dp[i1 + 1][i2].max(dp[i1][i2 + 1]);
            }
        }
    }
    let mut res = vec![];
    let (mut i1, mut i2) = (n1, n2);
    while i1 > 0 && i2 > 0 {
        if s1[i1 - 1] == s2[i2 - 1] {
            res.push(s1[i1 - 1]);
            i1 -= 1;
            i2 -= 1;
        } else if dp[i1 - 1][i2] > dp[i1][i2 - 1] {
            res.push(s1[i1 - 1]);
            i1 -= 1;
        } else {
            res.push(s2[i2 - 1]);
            i2 -= 1;
        }
    }
    if i1 > 0 {
        res.extend(s1[..i1].iter().rev());
    }
    if i2 > 0 {
        res.extend(s2[..i2].iter().rev());
    }
    // let (mut i1, mut i2) = (0, 0);
    // for b in find_lcs(s1, s2) {
    //     while i1 < s1.len() && s1[i1] != b {
    //         res.push(s1[i1]);
    //         i1 += 1;
    //     }
    //     while i2 < s2.len() && s2[i2] != b {
    //         res.push(s2[i2]);
    //         i2 += 1;
    //     }
    //     res.push(b);
    //     i1 += 1;
    //     i2 += 1;
    // }
    // if i1 < s1.len() {
    //     res.extend_from_slice(&s1[i1..]);
    // }
    // if i2 < s2.len() {
    //     res.extend_from_slice(&s2[i2..]);
    // }
    String::from_utf8(res).unwrap()
}

fn find_lcs(s1: &[u8], s2: &[u8]) -> Vec<u8> {
    let [n1, n2] = [s1, s2].map(|s| s.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for (i1, &b1) in s1.iter().enumerate() {
        for (i2, &b2) in s2.iter().enumerate() {
            if b1 == b2 {
                dp[i1 + 1][i2 + 1] = 1 + dp[i1][i2];
            } else {
                dp[i1 + 1][i2 + 1] = dp[i1 + 1][i2].max(dp[i1][i2 + 1]);
            }
        }
    }
    let mut res = Vec::with_capacity(dp[n1][n2]);
    let (mut i1, mut i2) = (n1, n2);
    while i1 > 0 && i2 > 0 {
        if s1[i1 - 1] == s2[i2 - 1] {
            res.push(s1[i1 - 1]);
            i1 -= 1;
            i2 -= 1;
        } else if dp[i1 - 1][i2] > dp[i1][i2 - 1] {
            i1 -= 1;
        } else {
            i2 -= 1;
        }
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(shortest_common_supersequence("abac", "cab"), "cabac");
        debug_assert_eq!(
            shortest_common_supersequence("aaaaaaaa", "aaaaaaaa"),
            "aaaaaaaa"
        );
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
