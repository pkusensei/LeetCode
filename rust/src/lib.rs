mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyCircularQueue {
    data: Box<[Option<i32>]>,
    cap: usize,
    r_p: usize,
    w_p: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            data: vec![None; k as usize].into_boxed_slice(),
            cap: k as usize,
            r_p: 0,
            w_p: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.data[self.w_p].is_some() {
            false
        } else {
            self.data[self.w_p] = Some(value);
            self.w_p = (1 + self.w_p) % self.cap;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.data[self.r_p].is_none() {
            false
        } else {
            self.data[self.r_p] = None;
            self.r_p = (1 + self.r_p) % self.cap;
            true
        }
    }

    fn front(&self) -> i32 {
        self.data[self.r_p].unwrap_or(-1)
    }

    fn rear(&self) -> i32 {
        let i = self.w_p.checked_sub(1).unwrap_or(self.cap - 1);
        self.data[i].unwrap_or(-1)
    }

    fn is_empty(&self) -> bool {
        self.w_p == self.r_p && self.data[self.r_p].is_none()
    }

    fn is_full(&self) -> bool {
        self.w_p == self.r_p && self.data[self.r_p].is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut myCircularQueue = MyCircularQueue::new(3);
        debug_assert!(myCircularQueue.en_queue(1)); // return True
        debug_assert!(myCircularQueue.en_queue(2)); // return True
        debug_assert!(myCircularQueue.en_queue(3)); // return True
        debug_assert!(!myCircularQueue.en_queue(4)); // return False
        myCircularQueue.rear(); // return 3
        debug_assert!(myCircularQueue.is_full()); // return True
        debug_assert!(myCircularQueue.de_queue()); // return True
        debug_assert!(myCircularQueue.en_queue(4)); // return True
        debug_assert_eq!(myCircularQueue.rear(), 4); // return 4
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
