mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn schedule_course(courses: &mut [[i32; 2]]) -> i32 {
    courses.sort_unstable_by_key(|v| v[1]);
    let mut total = 0;
    let mut heap = BinaryHeap::new();
    for v in courses.iter() {
        // Expand v as [dur, ddl]
        // Every dur that's pushed onto the heap has passed the check
        // total + dur <= ddl
        // Since array is sorted, every upcoming ddl must be bigger
        // Whenever a false check shows up
        // Try find a bigger __used__ dur on heap and swap it out
        // Thus reducing total
        if total + v[0] <= v[1] {
            total += v[0];
            heap.push(v[0]);
        } else if heap.peek().is_some_and(|&n| n > v[0]) {
            let Some(prev) = heap.pop() else {
                continue;
            };
            total -= prev;
            total += v[0];
            heap.push(v[0]);
        }
    }
    heap.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            schedule_course(&mut [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]),
            3
        );
        debug_assert_eq!(schedule_course(&mut [[1, 2]]), 1);
        debug_assert_eq!(schedule_course(&mut [[3, 2], [4, 3]]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(schedule_course(&mut [[1, 2], [2, 3]]), 2);
        debug_assert_eq!(schedule_course(&mut [[5, 5], [4, 6], [2, 6]]), 2);
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
