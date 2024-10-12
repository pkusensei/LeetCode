mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn falling_squares(positions: &[[i32; 2]]) -> Vec<i32> {
    let mut map: BTreeMap<[i32; 2], i32> = BTreeMap::new();
    let mut res = vec![];
    let mut height = 0;
    for item in positions.iter() {
        let (start, end) = (item[0], item[0] + item[1]);
        let mut curr_height = item[1];
        let mut temp = 0;
        for (&[left, right], &h) in map.iter() {
            if end <= left || right <= start {
                continue;
            }
            temp = temp.max(h);
        }
        curr_height += temp;
        height = height.max(curr_height);
        map.insert([start, end], curr_height);
        res.push(height);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(falling_squares(&[[1, 2], [2, 3], [6, 1]]), [2, 5, 5]);
        debug_assert_eq!(falling_squares(&[[100, 100], [200, 100]]), [100, 100]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(falling_squares(&[[7, 1], [3, 3], [7, 5]]), [1, 3, 6]);
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
