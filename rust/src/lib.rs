mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

struct LFUCache {
    cap: usize,
    kvc: HashMap<i32, (i32, i32)>,
    ck: BTreeMap<i32, VecDeque<i32>>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            kvc: HashMap::new(),
            ck: BTreeMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.cap == 0 {
            return -1;
        }
        if let Some(vc) = self.kvc.get_mut(&key) {
            let count = vc.1;
            let q = self.ck.get_mut(&count).unwrap();
            q.retain(|&k| k != key);
            if q.is_empty() {
                self.ck.remove(&count);
            }
            self.ck.entry(count + 1).or_default().push_back(key);
            vc.1 += 1;
            return vc.0;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        if let Some(vc) = self.kvc.get_mut(&key) {
            vc.0 = value;
            self.get(key);
        } else {
            if self.kvc.len() == self.cap {
                let (&count, q) = self.ck.iter_mut().next().unwrap();
                let del_key = q.pop_front().unwrap();
                if q.is_empty() {
                    self.ck.remove(&count);
                }
                self.kvc.remove(&del_key);
            }
            self.kvc.insert(key, (value, 1));
            self.ck.entry(1).or_default().push_back(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1); // cache=[1,_], cnt(1)=1
        lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
        debug_assert_eq!(lfu.get(1), 1); // return 1
                                         // cache=[1,2], cnt(2)=1, cnt(1)=2
        lfu.put(3, 3); // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                       // cache=[3,1], cnt(3)=1, cnt(1)=2
        debug_assert_eq!(lfu.get(2), -1); // return -1 (not found)
        debug_assert_eq!(lfu.get(3), 3); // return 3
                                         // cache=[3,1], cnt(3)=2, cnt(1)=2
        lfu.put(4, 4); // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                       // cache=[4,3], cnt(4)=1, cnt(3)=2
        debug_assert_eq!(lfu.get(1), -1); // return -1 (not found)
        debug_assert_eq!(lfu.get(3), 3); // return 3
                                         // cache=[3,4], cnt(4)=1, cnt(3)=3
        debug_assert_eq!(lfu.get(4), 4); // return 4
                                         // cache=[4,3], cnt(4)=2, cnt(3)=3
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
