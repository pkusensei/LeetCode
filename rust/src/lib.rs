mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_relative_ranks(score: &[i32]) -> Vec<String> {
    let mut nums: Vec<_> = score.iter().copied().enumerate().collect();
    nums.sort_unstable_by_key(|p| std::cmp::Reverse(p.1));
    let mut res: Vec<_> = nums
        .into_iter()
        .enumerate()
        .map(|(rank, (idx, _))| {
            let s = match rank {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                n => (n + 1).to_string(),
            };
            (idx, s)
        })
        .collect();
    res.sort_unstable_by_key(|p| p.0);
    res.into_iter().map(|p| p.1).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_relative_ranks(&[5, 4, 3, 2, 1]),
            ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
        debug_assert_eq!(
            find_relative_ranks(&[10, 3, 8, 9, 4]),
            ["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
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
}
