mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct CustomStack {
    data: Vec<i32>,
    cap: usize,
}

impl CustomStack {
    fn new(cap: i32) -> Self {
        let cap = cap as usize;
        Self {
            data: Vec::with_capacity(cap),
            cap,
        }
    }

    fn push(&mut self, x: i32) {
        if self.data.len() < self.cap {
            self.data.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..k as usize {
            if let Some(v) = self.data.get_mut(i) {
                *v += val;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut stk = CustomStack::new(3); // Stack is Empty []
        stk.push(1); // stack becomes [1]
        stk.push(2); // stack becomes [1, 2]
        debug_assert_eq!(stk.pop(), 2); // return 2 --> Return top of the stack 2, stack becomes [1]
        stk.push(2); // stack becomes [1, 2]
        stk.push(3); // stack becomes [1, 2, 3]
        stk.push(4); // stack still [1, 2, 3], Do not add another elements as size is 4
        stk.increment(5, 100); // stack becomes [101, 102, 103]
        stk.increment(2, 100); // stack becomes [201, 202, 103]
        debug_assert_eq!(stk.pop(), 103); // return 103 --> Return top of the stack 103, stack becomes [201, 202]
        debug_assert_eq!(stk.pop(), 202); // return 202 --> Return top of the stack 202, stack becomes [201]
        debug_assert_eq!(stk.pop(), 201); // return 201 --> Return top of the stack 201, stack becomes []
        debug_assert_eq!(stk.pop(), -1); // return -1 --> Stack is empty return -1.
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
