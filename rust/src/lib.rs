mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use rand::Rng;

#[derive(Debug, Clone)]
struct Solution {
    rects: Vec<[i32; 4]>,
    prefix: Vec<i32>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let rects: Vec<_> = rects.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let mut prefix = Vec::with_capacity(rects.len());
        prefix.push(Self::area_of(rects[0]));
        for (i, &rect) in rects.iter().enumerate().skip(1) {
            prefix.push(prefix[i - 1] + Self::area_of(rect));
        }
        Self { rects, prefix }
    }

    fn pick(&self) -> Vec<i32> {
        let upper = *self.prefix.last().unwrap();
        let mut rng = rand::thread_rng();
        let area = rng.gen_range(1..=upper);
        let i = self.prefix.partition_point(|&n| n < area);
        let rect = self.rects[i];
        vec![
            rng.gen_range(rect[0]..=rect[2]),
            rng.gen_range(rect[1]..=rect[3]),
        ]
    }

    fn area_of(rect: [i32; 4]) -> i32 {
        (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let solution = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
        solution.pick(); // return [1, -2]
        solution.pick(); // return [1, -1]
        solution.pick(); // return [-1, -2]
        solution.pick(); // return [-2, -2]
        solution.pick(); // return [0, 0]// return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
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
