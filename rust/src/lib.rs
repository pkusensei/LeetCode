mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn intersection_size_two(intervals: &mut [[i32; 2]]) -> i32 {
    // sort by increading end value
    // then by decreasing start value
    intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(b[0].cmp(&a[0])));
    let mut right_most = intervals[0][1];
    let mut second_right = right_most - 1;
    let mut res = 2;
    for v in intervals.iter().skip(1) {
        let (start, end) = (v[0], v[1]);
        if start <= second_right && start <= right_most {
            // [1,4] and [2,5] with [3,4] as [second_right,right_most]
            // take both second_right and right_most, i.e 3,4
            continue;
        }
        if start <= right_most {
            // [1,4] and [4,6] with [3,4] as [second_right,right_most]
            // take right_most, i.e 4
            second_right = right_most;
            right_most = end;
            res += 1;
        } else {
            // no overlapping, reset
            res += 2;
            right_most = end;
            second_right = right_most - 1;
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
        debug_assert_eq!(intersection_size_two(&mut [[1, 3], [3, 7], [8, 9]]), 5);
        debug_assert_eq!(
            intersection_size_two(&mut [[1, 3], [1, 4], [2, 5], [3, 5]]),
            3
        );
        debug_assert_eq!(
            intersection_size_two(&mut [[1, 2], [2, 3], [2, 4], [4, 5]]),
            5
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            intersection_size_two(&mut [
                [2, 10],
                [3, 7],
                [3, 15],
                [4, 11],
                [6, 12],
                [6, 16],
                [7, 8],
                [7, 11],
                [7, 15],
                [11, 12]
            ]),
            5
        );
        debug_assert_eq!(
            intersection_size_two(&mut [[1, 3], [3, 7], [5, 7], [7, 8]]),
            5
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
