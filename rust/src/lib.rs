mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut free: BinaryHeap<_> = servers
        .iter()
        .enumerate()
        .map(|(i, &w)| Reverse((w, i, 0)))
        .collect();
    let mut busy = BinaryHeap::new();
    let mut res = Vec::with_capacity(tasks.len());
    for (ti, &task) in tasks.iter().enumerate() {
        while let Some(&Reverse((time, w, i))) = busy.peek()
            && time <= ti as i32
        {
            free.push(Reverse((w, i, time)));
            busy.pop();
        }
        if let Some(Reverse((w, i, time))) = free.pop() {
            res.push(i as i32);
            busy.push(Reverse((time.max(ti as i32) + task, w, i)));
        } else if let Some(Reverse((time, w, i))) = busy.pop() {
            res.push(i as i32);
            busy.push(Reverse((time.max(ti as i32) + task, w, i)));
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {}

    #[test]
    fn test() {}
}
