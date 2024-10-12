mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_shortest_sub_array(nums: &[i32]) -> i32 {
    let map = nums.iter().enumerate().fold(
        HashMap::new(),
        |mut acc: HashMap<i32, Vec<_>>, (i, &num)| {
            acc.entry(num).or_default().push(i);
            acc
        },
    );
    let count = map.values().map(|v| v.len()).max().unwrap_or(0);
    map.into_values()
        .filter_map(|v| {
            if v.len() == count {
                Some(1 + v.last().unwrap() - v.first().unwrap())
            } else {
                None
            }
        })
        .min()
        .unwrap() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_shortest_sub_array(&[1, 2, 2, 3, 1]), 2);
        debug_assert_eq!(find_shortest_sub_array(&[1, 2, 2, 3, 1, 4, 2]), 6);
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
