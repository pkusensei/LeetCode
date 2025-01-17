mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct TreeAncestor {
    jump: Vec<Vec<i32>>,
    max_pow: usize,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let max_pow = 1 + n.ilog2() as usize;
        let mut jump = vec![vec![0; n]; max_pow];
        jump[0] = parent;
        for p in 1..max_pow {
            for idx in 0..n {
                let pre = jump[p - 1][idx];
                jump[p][idx] = if pre == -1 {
                    -1
                } else {
                    jump[p - 1][pre as usize]
                };
            }
        }
        Self { jump, max_pow }
    }

    fn get_kth_ancestor(&self, mut node: i32, mut k: i32) -> i32 {
        let mut max_pow = self.max_pow;
        while k > 0 && node > -1 {
            if k >= (1 << max_pow) {
                node = self.jump[max_pow][node as usize];
                k -= 1 << max_pow;
            } else {
                max_pow -= 1;
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let ta = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(ta.get_kth_ancestor(3, 1), 1); // returns 1 which is the parent of 3
        assert_eq!(ta.get_kth_ancestor(5, 2), 0); // returns 0 which is the grandparent of 5
        assert_eq!(ta.get_kth_ancestor(6, 3), -1); // returns -1 because there is no such ancestor
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
