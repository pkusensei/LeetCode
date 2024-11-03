mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

struct RLEIterator {
    data: VecDeque<(i32, i32)>,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self {
            data: encoding
                .chunks_exact(2)
                .filter_map(|ch| {
                    if ch[0] > 0 {
                        Some((ch[0], ch[1]))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    fn next(&mut self, mut n: i32) -> i32 {
        while self.data.front().is_some_and(|&(count, _)| count < n) {
            let Some((count, _)) = self.data.pop_front() else {
                return -1;
            };
            n -= count;
        }
        let Some((mut count, num)) = self.data.pop_front() else {
            return -1;
        };
        count -= n;
        if count > 0 {
            self.data.push_front((count, num));
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut it = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]); // This maps to the sequence [8,8,8,5,5].
        debug_assert_eq!(it.next(2), 8); // exhausts 2 terms of the sequence, returning 8. The remaining sequence is now [8, 5, 5].
        debug_assert_eq!(it.next(1), 8); // exhausts 1 term of the sequence, returning 8. The remaining sequence is now [5, 5].
        debug_assert_eq!(it.next(1), 5); // exhausts 1 term of the sequence, returning 5. The remaining sequence is now [5].
        debug_assert_eq!(it.next(2), -1); // exhausts 2 terms, returning -1. This is because the first term exhausted was 5,
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
