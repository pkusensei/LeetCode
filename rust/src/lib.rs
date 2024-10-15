mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyCalendar {
    data: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((&_left, &right)) = self.data.range(..end).next_back() {
            if start < right {
                return false;
            }
        }
        self.data.insert(start, end).is_none()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut cal = MyCalendar::new();
        debug_assert!(cal.book(10, 20)); // return True
        debug_assert!(!cal.book(15, 25)); // return False, It can not be booked because time 15 is already booked by another event.
        debug_assert!(cal.book(20, 30)); // return True
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
