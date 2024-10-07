mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn is_possible(nums: &[i32]) -> bool {
    let mut counts = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    for &n in nums.iter() {
        let mut curr = counts.get(&n).copied().unwrap_or(0);
        if curr == 0 {
            continue;
        };
        let mut count = 0;
        let mut num = n;
        while let Some(&c) = counts.get(&num) {
            // For each start, there must be an end that satisfies
            // count(end) >= count(start)
            // Otherwise it is in valid
            if c < curr {
                break;
            }
            curr = c;
            counts.entry(num).and_modify(|v| *v -= 1);
            count += 1;
            num += 1
        }
        if count < 3 {
            return false;
        }
    }
    true
}

fn with_pq(nums: &[i32]) -> bool {
    let mut heap = BinaryHeap::new();
    for &num in nums.iter() {
        loop {
            match heap.peek().copied() {
                // queue is empty; start a new seq
                None => {
                    heap.push(Reverse((num, 1)));
                    break;
                }
                // num added to existed seq
                Some(Reverse((n, c))) if n == num - 1 => {
                    heap.pop();
                    heap.push(Reverse((num, c + 1)));
                    break;
                }
                // start a new seq
                Some(Reverse((n, _))) if n == num => {
                    heap.push(Reverse((num, 1)));
                    break;
                }
                // a seq concluded
                Some(Reverse((_, c))) => {
                    if c < 3 {
                        return false;
                    } else {
                        heap.pop();
                    }
                }
            }
        }
    }
    heap.into_iter().all(|Reverse((_, c))| c >= 3)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(with_pq(&[1, 2, 3, 3, 4, 5]));
        debug_assert!(with_pq(&[1, 2, 3, 3, 4, 4, 5, 5]));
        debug_assert!(!with_pq(&[1, 2, 3, 4, 4, 5]));
    }

    #[test]
    fn test() {
        debug_assert!(with_pq(&[1, 2, 3, 4, 6, 7, 8, 9, 10, 11]));
        debug_assert!(with_pq(&[1, 2, 3, 4, 5, 5, 6, 7]));
    }

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
