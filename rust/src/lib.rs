mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyCalendarTwo {
    counts: BTreeMap<i32, i32>,
    max_overlap: i32,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            counts: BTreeMap::new(),
            max_overlap: 2,
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.counts.entry(start).or_insert(0) += 1;
        *self.counts.entry(end).or_insert(0) -= 1;
        let mut overlap = 0;
        for v in self.counts.values() {
            overlap += v;
            if overlap > self.max_overlap {
                break;
            }
        }
        if overlap > self.max_overlap {
            self.counts.entry(start).and_modify(|v| *v -= 1);
            self.counts.entry(end).and_modify(|v| *v += 1);
            if self.counts.get(&start).is_some_and(|&v| v == 0) {
                self.counts.remove(&start);
            };
            if self.counts.get(&end).is_some_and(|&v| v == 0) {
                self.counts.remove(&end);
            };
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut cal = MyCalendarTwo::new();
        debug_assert!(cal.book(10, 20)); // return True, The event can be booked.
        debug_assert!(cal.book(50, 60)); // return True, The event can be booked.
        debug_assert!(cal.book(10, 40)); // return True, The event can be double booked.
        debug_assert!(!cal.book(5, 15)); // return False, The event cannot be booked, because it would result in a triple booking.
        debug_assert!(cal.book(5, 10)); // return True, The event can be booked, as it does not use time 10 which is already double booked.
        debug_assert!(cal.book(25, 55)); // return True, The event can be booked, as the time in [25, 40) will be double booked with the third event, the time [40, 50) will be single booked, and the time [50, 55) will be double booked with the second event.
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
