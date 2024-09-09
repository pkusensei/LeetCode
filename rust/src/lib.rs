mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_self_crossing(distance: &[i32]) -> bool {
    if distance.len() < 4 {
        return false;
    }
    for i in 3..distance.len() {
        //  ---
        //  | |
        //  ---->[i]
        //    |
        //    *
        if distance[i - 3] >= distance[i - 1] && distance[i - 2] <= distance[i] {
            return true;
        }
        //  ----
        //  |  |
        //  |  *
        //  |  ^[i]
        //  |  |
        //  ----
        if i >= 4
            && distance[i - 3] == distance[i - 1]
            && distance[i - 4] + distance[i] >= distance[i - 2]
        {
            return true;
        }
        //  ------
        //  |    |
        //  |[i]<---
        //  |    | |
        //  |    * |
        //  --------
        if i >= 5
            && distance[i - 5] <= distance[i - 3]
            && distance[i - 1] <= distance[i - 3]
            && distance[i - 5] + distance[i - 1] >= distance[i - 3]
            && distance[i - 4] <= distance[i - 2]
            && distance[i] <= distance[i - 2]
            && distance[i - 4] + distance[i] >= distance[i - 2]
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_self_crossing(&[2, 1, 1, 2]));
        debug_assert!(!is_self_crossing(&[1, 2, 3, 4]));
        debug_assert!(is_self_crossing(&[1, 1, 1, 2, 1]));
    }

    #[test]
    fn test() {
        debug_assert!(is_self_crossing(&[1, 1, 2, 1, 1]));
        debug_assert!(!is_self_crossing(&[
            1, 1, 2, 2, 3, 3, 4, 4, 10, 4, 4, 3, 3, 2, 2, 1, 1
        ]));
    }

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
