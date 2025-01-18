mod dsu;
mod helper;
mod trie;

use std::collections::{BinaryHeap, VecDeque};

#[allow(unused_imports)]
use helper::*;

// yi + yj + |xi - xj|
// yj+xj + yi-xi
pub fn find_max_value_of_equation(points: &[[i32; 2]], k: i32) -> i32 {
    // [yi-xi, xi]
    let mut heap: BinaryHeap<[i32; 2]> = BinaryHeap::new();
    let mut res = i32::MIN;
    for p in points.iter() {
        while heap.peek().is_some_and(|v| p[0] - v[1] > k) {
            heap.pop();
        }
        if let Some(v) = heap.peek() {
            res = res.max(p[1] + p[0] + v[0]);
        }
        heap.push([p[1] - p[0], p[0]]);
    }
    res
}

fn with_deque(points: &[[i32; 2]], k: i32) -> i32 {
    let mut queue = VecDeque::<[i32; 2]>::new();
    let mut res = 0;
    for p in points.iter() {
        while queue.front().is_some_and(|v| p[0] - v[1] > k) {
            queue.pop_front();
        }
        if let Some(v) = queue.front() {
            res = res.max(p[0] + p[1] + v[0]);
        }
        while queue.back().is_some_and(|v| v[0] <= p[1] - p[0]) {
            queue.pop_back();
        }
        queue.push_back([p[1] - p[0], p[0]]);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_max_value_of_equation(&[[1, 3], [2, 0], [5, 10], [6, -10]], 1),
            4
        );
        assert_eq!(find_max_value_of_equation(&[[0, 0], [3, 0], [9, 2]], 3), 3);

        assert_eq!(with_deque(&[[1, 3], [2, 0], [5, 10], [6, -10]], 1), 4);
        assert_eq!(with_deque(&[[0, 0], [3, 0], [9, 2]], 3), 3);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
