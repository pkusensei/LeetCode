mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyCircularDeque {
    data: Box<[i32]>,
    cap: usize,
    size: usize,
    front: usize,
    back: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let cap = k as usize;
        Self {
            data: vec![0; cap].into_boxed_slice(),
            cap,
            size: 0,
            front: 0,
            back: cap - 1,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.front = self.front.checked_sub(1).unwrap_or(self.cap - 1);
            self.data[self.front] = value;
            self.size += 1;
            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.back = (self.back + 1) % self.cap;
            self.data[self.back] = value;
            self.size += 1;
            true
        }
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.front = (self.front + 1) % self.cap;
            self.size -= 1;
            true
        }
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.back = self.back.checked_sub(1).unwrap_or(self.cap - 1);
            self.size -= 1;
            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.back]
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn is_full(&self) -> bool {
        self.size == self.cap
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut q = MyCircularDeque::new(3);
        debug_assert!(q.insert_last(1)); // return True
        debug_assert!(q.insert_last(2)); // return True
        debug_assert!(q.insert_front(3)); // return True
        debug_assert!(!q.insert_front(4)); // return False, the queue is full.
        debug_assert_eq!(q.get_rear(), 2); // return 2
        debug_assert!(q.is_full()); // return True
        debug_assert!(q.delete_last()); // return True
        debug_assert!(q.insert_front(4)); // return True
        debug_assert_eq!(q.get_front(), 4); // return 4
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
