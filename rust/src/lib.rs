mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_distance(word: &str) -> i32 {
    // let s = word.as_bytes();
    // dfs(
    //     &s[1..],
    //     0,
    //     s[0],
    //     EMPTY,
    //     &mut vec![vec![vec![-1; 27]; 27]; s.len()],
    // )
    let mut curr = HashMap::from([((26, 26), 0)]);
    let dist = |a: i32, b: i32| {
        if a == 26 {
            0
        } else {
            ((a % 6).abs_diff(b % 6) + (a / 6).abs_diff(b / 6)) as i32
        }
    };
    for b in word.bytes().map(|b| i32::from(b - b'A')) {
        let mut next: HashMap<(i32, i32), i32> = HashMap::new();
        for ((x, y), count) in curr.into_iter() {
            next.entry((b, y))
                .and_modify(|v| *v = (*v).min(count + dist(x, b)))
                .or_insert(count + dist(x, b));
            next.entry((x, b))
                .and_modify(|v| (*v) = (*v).min(count + dist(y, b)))
                .or_insert(count + dist(y, b));
        }
        curr = next;
    }
    curr.into_values().min().unwrap()
}

const EMPTY: u8 = 64;

fn dfs(s: &[u8], idx: usize, b1: u8, b2: u8, memo: &mut [Vec<Vec<i32>>]) -> i32 {
    if idx == s.len() {
        return 0;
    }
    let [i1, i2] = [b1, b2].map(|v| usize::from(v - EMPTY));
    if memo[idx][i1][i2] > -1 {
        return memo[idx][i1][i2];
    }

    fn manhattan_dist(a: u8, b: u8) -> u32 {
        if a == EMPTY || b == EMPTY {
            return 0;
        }
        let [a, b] = [a, b].map(|v| i32::from(v - EMPTY - 1));
        (a % 6).abs_diff(b % 6) + (a / 6).abs_diff(b / 6)
    }

    let curr = s[idx];
    let dist1 = manhattan_dist(b1, curr) as i32 + dfs(s, 1 + idx, curr, b2, memo);
    let dist2 = manhattan_dist(b2, curr) as i32 + dfs(s, 1 + idx, b1, curr, memo);
    let res = dist1.min(dist2) as i32;
    memo[idx][i1][i2] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(minimum_distance("CAKE"), 3);
        assert_eq!(minimum_distance("HAPPY"), 6);
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
