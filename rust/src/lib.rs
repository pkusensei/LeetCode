mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn smallest_chair(times: &mut [[i32; 2]], target: i32) -> i32 {
    let target_arr = times[target as usize][0];
    times.sort_unstable_by_key(|t| t[0]);
    let (mut free, mut used) = (BinaryHeap::new(), BinaryHeap::new());

    for t in times.iter() {
        let (arr, dep) = (t[0], t[1]);
        while used.peek().is_some_and(|&(Reverse(t), _)| t <= arr) {
            let Some((_, ch)) = used.pop() else {
                break;
            };
            free.push(Reverse(ch));
        }
        let ch = free.pop().map(|ch| ch.0).unwrap_or(used.len() as i32);
        if target_arr == arr {
            return ch;
        }
        used.push((Reverse(dep), ch));
    }
    0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_chair(&mut [[1, 4], [2, 3], [4, 6]], 1), 1);
        debug_assert_eq!(smallest_chair(&mut [[3, 10], [1, 5], [2, 6]], 0), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            smallest_chair(
                &mut [
                    [33889, 98676],
                    [80071, 89737],
                    [44118, 52565],
                    [52992, 84310],
                    [78492, 88209],
                    [21695, 67063],
                    [84622, 95452],
                    [98048, 98856],
                    [98411, 99433],
                    [55333, 56548],
                    [65375, 88566],
                    [55011, 62821],
                    [48548, 48656],
                    [87396, 94825],
                    [55273, 81868],
                    [75629, 91467]
                ],
                6
            ),
            2
        )
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
