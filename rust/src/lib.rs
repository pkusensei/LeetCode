mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap = std::collections::BinaryHeap::from(nums);
    let mut res = 0;
    for _ in 0..k {
        let Some(num) = heap.pop() else {
            break;
        };
        res += i64::from(num);
        let n = if num % 3 == 0 { num / 3 } else { 1 + num / 3 };
        heap.push(n);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_kelements([10, 10, 10, 10, 10].into(), 5), 50);
        debug_assert_eq!(max_kelements([1, 10, 3, 3, 3].into(), 3), 17);
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
