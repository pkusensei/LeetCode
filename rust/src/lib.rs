mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_sufficient_team(req_skills: &[&str], people: &[&[&str]]) -> Vec<i32> {
        let skills = req_skills
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, s)| {
                acc.insert(*s, i);
                acc
            });
        let nums: Vec<_> = people
            .iter()
            .map(|p| p.iter().map(|s| skills[s]).fold(0, |acc, v| acc | (1 << v)))
            .collect();
        let n = req_skills.len();
        // Initially, keep all people in
        let mut dp = vec![(1i64 << nums.len()) - 1; 1 << n];
        dp[0] = 0;
        for mask in 1..1 << n {
            for (idx, &num) in nums.iter().enumerate() {
                let rest = mask & (!num);
                if rest != mask {
                    let team = dp[rest] | (1 << idx);
                    if team.count_ones() < dp[mask].count_ones() {
                        dp[mask] = team;
                    }
                }
            }
        }
        let team = dp[(1 << n) - 1];
        let mut res = Vec::with_capacity(team.count_ones() as usize);
        for i in 0..nums.len() {
            if (team >> i) & 1 == 1 {
                res.push(i as i32);
            }
        }
        res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            smallest_sufficient_team(
                &["java", "nodejs", "reactjs"],
                &[&["java"], &["nodejs"], &["nodejs", "reactjs"]]
            ),
            [0, 2]
        );
        debug_assert_eq!(
            smallest_sufficient_team(
                &["algorithms", "math", "java", "reactjs", "csharp", "aws"],
                &[
                    &["algorithms", "math", "java"],
                    &["algorithms", "math", "reactjs"],
                    &["java", "csharp", "aws"],
                    &["reactjs", "csharp"],
                    &["csharp", "math"],
                    &["aws", "java"]
                ]
            ),
            [1, 2]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
