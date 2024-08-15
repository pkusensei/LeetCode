mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct MinStack {
    nums: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        self.nums.push(val);
        match self.mins.last() {
            Some(&m) if m >= val => self.mins.push(val),
            None => self.mins.push(val),
            _ => (),
        }
    }

    fn pop(&mut self) {
        if self.mins.last().copied() >= self.nums.pop() {
            self.mins.pop();
        }
    }

    fn top(&self) -> i32 {
        self.nums.last().copied().unwrap_or(0)
    }

    fn get_min(&self) -> i32 {
        self.mins.last().copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        debug_assert_eq!(stack.get_min(), -3); // return -3
        stack.pop();
        stack.top(); // return 0
        debug_assert_eq!(stack.get_min(), -2); // return -2
    }

    #[test]
    fn test() {}
}
