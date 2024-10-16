use std::collections::BTreeMap;

mod helper;
mod trie;

#[derive(Debug, Clone, Default)]
struct MyCalendarThree {
    data: BTreeMap<i32, i32>,
    k: i32,
}

impl MyCalendarThree {
    fn new() -> Self {
        // Default::default()
        Self {
            data: BTreeMap::from([(0, 0)]),
            k: 0,
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        // *self.data.entry(start).or_insert(0) += 1;
        // *self.data.entry(end).or_insert(0) -= 1;
        // let mut res = 0;
        // let mut curr = 0;
        // for v in self.data.values() {
        //     curr += v;
        //     res = res.max(curr);
        // }
        // res
        // From naive approach there's a hidden prefix sum
        // To collapse that onto the BTreeMap itself...
        let Some((_, &(mut curr))) = self.data.range(..=start).next_back() else {
            return 0; // unreachable
        };
        self.data.entry(start).or_insert(curr);
        for (_, v) in self.data.range_mut(start..end) {
            curr = *v;
            *v += 1;
            self.k = self.k.max(*v);
        }
        self.data.entry(end).or_insert(curr);
        self.k
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut cal = MyCalendarThree::new();
        debug_assert_eq!(cal.book(10, 20), 1); // return 1
        debug_assert_eq!(cal.book(50, 60), 1); // return 1
        debug_assert_eq!(cal.book(10, 40), 2); // return 2
        debug_assert_eq!(cal.book(5, 15), 3); // return 3
        debug_assert_eq!(cal.book(5, 10), 3); // return 3
        debug_assert_eq!(cal.book(25, 55), 3); // return 3
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
