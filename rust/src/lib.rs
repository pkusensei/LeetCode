mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn watched_videos_by_friends(
    watched_videos: &[&[&str]],
    friends: &[&[i32]],
    id: i32,
    level: i32,
) -> Vec<String> {
    let n = watched_videos.len();
    let mut queue = VecDeque::from([(id as usize, 0)]);
    let mut seen = vec![false; n];
    seen[id as usize] = true;
    let mut count = HashMap::new();
    while let Some((curr, dist)) = queue.pop_front() {
        if dist == level {
            for &item in watched_videos[curr].iter() {
                *count.entry(item).or_insert(0) += 1;
            }
        }
        if dist > level {
            break;
        }
        for &next in friends[curr].iter() {
            let next = next as usize;
            if !seen[next] {
                seen[next] = true;
                queue.push_back((next, 1 + dist));
            }
        }
    }
    let mut res: Vec<_> = count.into_iter().collect();
    res.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(b.0)));
    res.into_iter().map(|(k, _)| k.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            watched_videos_by_friends(
                &[&["A", "B"], &["C"], &["B", "C"], &["D"]],
                &[&[1, 2], &[0, 3], &[0, 3], &[1, 2]],
                0,
                1
            ),
            ["B", "C"]
        );
        assert_eq!(
            watched_videos_by_friends(
                &[&["A", "B"], &["C"], &["B", "C"], &["D"]],
                &[&[1, 2], &[0, 3], &[0, 3], &[1, 2]],
                0,
                2
            ),
            ["D"]
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
