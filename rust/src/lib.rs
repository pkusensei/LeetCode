mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct MyQueue {
    ins: Vec<i32>,
    outs: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        while let Some(v) = self.outs.pop() {
            self.ins.push(v)
        }
        self.ins.push(x);
        while let Some(v) = self.ins.pop() {
            self.outs.push(v)
        }
    }

    fn pop(&mut self) -> i32 {
        self.outs.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.outs.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.outs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut queue = MyQueue::new();
        queue.push(1); // queue is: [1]
        queue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
        debug_assert_eq!(queue.peek(), 1); // return 1
        debug_assert_eq!(queue.pop(), 1); // return 1, queue is [2]
        debug_assert!(!queue.empty()); // return false
    }

    #[test]
    fn test() {}
}
