mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_min_arrow_shots(mut points: Vec<[i32; 2]>) -> i32 {
    points.sort_unstable_by_key(|v| v[1]);
    let n = points.len();
    if n < 2 {
        return 1;
    }
    let mut curr = points[0][1];
    let mut res = 1;
    for p in points.iter().skip(1) {
        if p[0] > curr {
            res += 1;
            curr = p[1]
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
            find_min_arrow_shots(vec![[10, 16], [2, 8], [1, 6], [7, 12]]),
            2
        );
        debug_assert_eq!(
            find_min_arrow_shots(vec![[1, 2], [3, 4], [5, 6], [7, 8]]),
            4
        );
        debug_assert_eq!(
            find_min_arrow_shots(vec![[1, 2], [2, 3], [3, 4], [4, 5]]),
            2
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            find_min_arrow_shots(vec![
                [9, 12],
                [1, 10],
                [4, 11],
                [8, 12],
                [3, 9],
                [6, 9],
                [6, 7]
            ]),
            2
        );
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
