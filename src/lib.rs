use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct KthLargest {
    k: usize,
    nums: BinaryHeap<std::cmp::Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut nums: BinaryHeap<_> = nums.into_iter().map(std::cmp::Reverse).collect();
        while nums.len() > k {
            nums.pop(); // only k elements necessary to keep
        }
        Self { k, nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(std::cmp::Reverse(val));
        while self.nums.len() > self.k {
            self.nums.pop();
        }
        self.nums.peek().unwrap().0
    }
}

// fn is_palindrome(s: &str) -> bool {
//     if s.len() < 2 {
//         return true;
//     }
//     s.bytes()
//         .rev()
//         .zip(s.bytes().take(s.len() / 2 + 1))
//         .all(|(b1, b2)| b1 == b2)
// }

// type Coord = (usize, usize);

// fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
//     [
//         (a.saturating_sub(1), b),
//         (a + 1, b),
//         (a, b.saturating_sub(1)),
//         (a, b + 1),
//     ]
//     .into_iter()
//     .filter(move |&p| p != (a, b))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
            debug_assert_eq!(kth.add(3), 4);
            debug_assert_eq!(kth.add(5), 5);
            debug_assert_eq!(kth.add(10), 5);
            debug_assert_eq!(kth.add(9), 8);
            debug_assert_eq!(kth.add(4), 8);
        }
    }

    #[test]
    fn test() {}
}
