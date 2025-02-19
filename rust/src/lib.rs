mod dsu;
mod helper;
mod trie;

use std::cell::Cell;

#[allow(unused_imports)]
use helper::*;

struct LockingTree {
    states: Vec<Cell<i32>>,
    parent: Vec<i32>,
    adj: Vec<Vec<usize>>,
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let n = parent.len();
        let adj = parent
            .iter()
            .enumerate()
            .fold(vec![vec![]; n], |mut acc, (node, &p)| {
                if p >= 0 {
                    acc[p as usize].push(node);
                }
                acc
            });
        Self {
            states: vec![Cell::new(0); n],
            parent,
            adj,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        let node = num as usize;
        if self.states[node].get() == 0 {
            self.states[node].set(user);
            return true;
        }
        false
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let node = num as usize;
        if self.states[node].get() == user {
            self.states[node].set(0);
            return true;
        }
        false
    }

    fn upgrade(&self, num: i32, user: i32) -> bool {
        let node = num as usize;
        if self.states[node].get() > 0 {
            return false;
        }
        // Find potentially locked acestor
        let mut p = self.parent[node];
        while p >= 0 {
            if self.states[p as usize].get() > 0 {
                return false;
            }
            p = self.parent[p as usize];
        }
        if self.unlock_desc(node) {
            self.states[node].set(user);
            return true;
        }
        false
    }

    fn unlock_desc(&self, node: usize) -> bool {
        let mut res = false;
        for &next in self.adj[node].iter() {
            if self.states[next].get() > 0 {
                self.states[next].set(0);
                res |= true
            }
            res |= self.unlock_desc(next);
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        let mut lt = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
        assert!(lt.lock(2, 2)); // return true because node 2 is unlocked.
                                // Node 2 will now be locked by user 2.
        assert!(!lt.unlock(2, 3)); // return false because user 3 cannot unlock a node locked by user 2.
        assert!(lt.unlock(2, 2)); // return true because node 2 was previously locked by user 2.
                                  // Node 2 will now be unlocked.
        assert!(lt.lock(4, 5)); // return true because node 4 is unlocked.
                                // Node 4 will now be locked by user 5.
        assert!(lt.upgrade(0, 1)); // return true because node 0 is unlocked and has at least one locked descendant (node 4).
                                   // Node 0 will now be locked by user 1 and node 4 will now be unlocked.
        assert!(!lt.lock(0, 1)); // return false because node 0 is already locked.
    }

    #[test]
    fn test() {}
}
