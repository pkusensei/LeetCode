mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct RangeModule {
    data: BTreeSet<[i32; 2]>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            data: BTreeSet::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let [left_in, right_in] = [left, right].map(|num| {
            self.data
                .iter()
                .copied()
                .find(|v| (v[0]..=v[1]).contains(&num))
        });
        let dels: Vec<_> = self
            .data
            .iter()
            .filter(|&int| left < int[0] && int[1] < right)
            .copied()
            .collect();
        for int in dels {
            self.data.remove(&int);
        }
        match (left_in, right_in) {
            (None, None) => {
                self.data.insert([left, right]);
            }
            (Some(interval), None) => {
                self.data.remove(&interval);
                self.data.insert([interval[0], right]);
            }
            (None, Some(interval)) => {
                self.data.remove(&interval);
                self.data.insert([left, interval[1]]);
            }
            (Some(int1), Some(int2)) if int1 != int2 => {
                debug_assert!(int1 < int2);
                self.data.remove(&int1);
                self.data.remove(&int2);
                self.data.insert([int1[0], int2[1]]);
            }
            _ => (),
        }
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.data
            .iter()
            .any(|v| (v[0]..v[1]).contains(&left) && (v[0]..=v[1]).contains(&right))
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let [left_in, right_in] = [left, right].map(|num| {
            self.data
                .iter()
                .copied()
                .find(|v| (v[0]..=v[1]).contains(&num))
        });
        let dels: Vec<_> = self
            .data
            .iter()
            .copied()
            .filter(|&int| left < int[0] && int[1] < right)
            .collect();
        for int in dels {
            self.data.remove(&int);
        }
        match (left_in, right_in) {
            (Some(interval), None) => {
                self.data.remove(&interval);
                if left > interval[0] {
                    self.data.insert([interval[0], left]);
                }
            }
            (None, Some(internal)) => {
                self.data.remove(&internal);
                if right < internal[1] {
                    self.data.insert([right, internal[1]]);
                }
            }
            (Some(int1), Some(int2)) if int1 == int2 => {
                self.data.remove(&int1);
                if left > int1[0] {
                    self.data.insert([int1[0], left]);
                }
                if right < int2[1] {
                    self.data.insert([right, int2[1]]);
                }
            }
            (Some(int1), Some(int2)) => {
                self.data.remove(&int1);
                self.data.remove(&int2);
                if left > int1[0] {
                    self.data.insert([int1[0], left]);
                }
                if right < int2[1] {
                    self.data.insert([right, int2[1]]);
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut range = RangeModule::new();
        range.add_range(10, 20);
        range.remove_range(14, 16);
        debug_assert!(range.query_range(10, 14)); // return True,(Every number in [10, 14) is being tracked)
        debug_assert!(!range.query_range(13, 15)); // return False,(Numbers like 14, 14.03, 14.17 in [13, 15) are not being tracked)
        debug_assert!(range.query_range(16, 17)); // return True,
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
