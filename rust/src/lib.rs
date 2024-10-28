mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn is_n_straight_hand(hand: &[i32], size: i32) -> bool {
    if hand.len() % size as usize > 0 {
        return false;
    }
    let mut nums = hand.iter().fold(BTreeMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    while let Some(start) = nums.keys().next().copied() {
        let mut count = 0;
        for (_, v) in nums.range_mut(start..start + size) {
            *v -= 1;
            count += 1;
        }
        if count != size {
            return false;
        }
        nums.retain(|_, &mut v| v > 0);
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_n_straight_hand(&[1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
        debug_assert!(!is_n_straight_hand(&[1, 2, 3, 4, 5], 4));
    }

    #[test]
    fn test() {
        debug_assert!(is_n_straight_hand(&[1, 2, 3, 4, 5, 6], 2));
    }

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
