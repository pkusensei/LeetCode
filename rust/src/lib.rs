mod helper;

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct AllOne {
    strs: HashMap<String, usize>,
    counts: BTreeSet<(usize, String)>,
}

impl AllOne {
    fn new() -> Self {
        Default::default()
    }

    fn inc(&mut self, key: String) {
        if let Some(count) = self.strs.get_mut(&key) {
            *count += 1;
            self.counts.remove(&(*count - 1, key.clone()));
            self.counts.insert((*count, key));
        } else {
            self.strs.insert(key.clone(), 1);
            self.counts.insert((1, key));
        }
    }

    fn dec(&mut self, key: String) {
        if let Some(count) = self.strs.get_mut(&key) {
            *count -= 1;
            let count = *count;
            if count == 0 {
                self.strs.remove(&key);
                self.counts.remove(&(1, key));
            } else {
                self.counts.remove(&(count + 1, key.clone()));
                self.counts.insert((count, key));
            }
        }
    }

    fn get_max_key(&self) -> String {
        self.counts
            .last()
            .map(|(_, s)| s.to_string())
            .unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.counts
            .first()
            .map(|(_, s)| s.to_string())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut tmp = AllOne::new();
        tmp.inc("hello".to_string());
        tmp.inc("hello".to_string());
        debug_assert_eq!(tmp.get_max_key(), "hello"); // return "hello"
        debug_assert_eq!(tmp.get_min_key(), "hello"); // return "hello"
        tmp.inc("leet".to_string());
        debug_assert_eq!(tmp.get_max_key(), "hello"); // return "hello"
        debug_assert_eq!(tmp.get_min_key(), "leet"); // return "leet"
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
