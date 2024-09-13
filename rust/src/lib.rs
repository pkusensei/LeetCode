mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn first_uniq_char(s: &str) -> i32 {
    let counts = s.bytes().enumerate().fold([(0, 0); 26], |mut acc, (i, b)| {
        let pos = usize::from(b - b'a');
        acc[pos].0 = i;
        acc[pos].1 += 1;
        acc
    });
    counts
        .into_iter()
        .filter_map(|(idx, count)| if count == 1 { Some(idx as i32) } else { None })
        .min()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(first_uniq_char("leetcode"), 0);
        debug_assert_eq!(first_uniq_char("loveleetcode"), 2);
        debug_assert_eq!(first_uniq_char("aabb"), -1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
