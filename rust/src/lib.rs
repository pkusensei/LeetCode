mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_longest_chain(pairs: &mut [[i32; 2]]) -> i32 {
    pairs.sort_unstable_by_key(|v| v[1]);
    let mut res = 1;
    let mut end = pairs[0][1];
    for p in pairs.iter().skip(1) {
        if p[0] > end {
            res += 1;
            end = p[1];
        }
    }
    res
    // with_dp(pairs)
}

fn with_dp(pairs: &[[i32; 2]]) -> i32 {
    let n = pairs.len();
    let mut dp = vec![1; n];
    for i in 1..n {
        for j in 0..n {
            if pairs[i][0] > pairs[j][1] {
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_longest_chain(&mut [[1, 2], [2, 3], [3, 4]]), 2);
        debug_assert_eq!(find_longest_chain(&mut [[1, 2], [7, 8], [4, 5]]), 3);
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
