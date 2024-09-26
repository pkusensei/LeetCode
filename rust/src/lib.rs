mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyCalendar {
    data: Vec<[i32; 2]>,
}

impl MyCalendar {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.data.is_empty() {
            self.data.push([start, end]);
            return true;
        }
        let Err(i) = self.data.binary_search_by(|v| v[0].cmp(&start)) else {
            return false;
        };
        if i == 0 && self.data.first().is_some_and(|v| end <= v[0]) {
            self.data.insert(0, [start, end]);
            true
        } else if i == self.data.len() && self.data.last().is_some_and(|v| v[1] <= start) {
            self.data.push([start, end]);
            true
        } else if self.data.get(i - 1).is_some_and(|v| v[1] <= start)
            && self.data.get(i).is_some_and(|v| end <= v[0])
        {
            self.data.insert(i, [start, end]);
            true
        } else {
            false
        }
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
        debug_assert!(cal.book(20, 30)); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
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
