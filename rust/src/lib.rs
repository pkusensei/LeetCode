mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn least_bricks(wall: &[&[i32]]) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for nums in wall.iter() {
        for num in prefix(nums) {
            *counts.entry(num).or_insert(0) += 1
        }
    }
    let mut counts: Vec<_> = counts.into_values().collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    counts[0] - counts.get(1).unwrap_or(&0)
}

fn prefix(nums: &[i32]) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(nums.len());
    for &num in nums.iter() {
        prefix.push(prefix.last().copied().unwrap_or(0) + num);
    }
    prefix
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            least_bricks(&[
                &[1, 2, 2, 1],
                &[3, 1, 2],
                &[1, 3, 2],
                &[2, 4],
                &[3, 1, 2],
                &[1, 3, 1, 1]
            ]),
            2
        );
        debug_assert_eq!(least_bricks(&[&[1], &[1], &[1]]), 3);
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
