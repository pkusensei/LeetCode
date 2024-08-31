mod helper;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(50),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 0..self.queue.len() - 1 {
            let t = self.queue.pop_front().unwrap();
            self.queue.push_back(t)
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut s = MyStack::new();
        s.push(1);
        s.push(2);
        debug_assert_eq!(s.top(), 2);
        debug_assert_eq!(s.pop(), 2);
        debug_assert!(!s.empty());
    }

    #[test]
    fn test() {}
}
