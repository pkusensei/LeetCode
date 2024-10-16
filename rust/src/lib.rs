mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut heap: BinaryHeap<_> = [a, b, c]
        .into_iter()
        .enumerate()
        .filter_map(|(i, num)| {
            if num > 0 {
                Some((num, char::from(i as u8 + b'a')))
            } else {
                None
            }
        })
        .collect();
    let mut res = vec![];
    while let Some((mut num, ch)) = heap.pop() {
        if res.ends_with(&[ch; 2]) {
            let Some((n, c)) = heap.pop() else {
                break;
            };
            res.push(c);
            if n > 1 {
                heap.push((n - 1, c));
            }
        } else {
            res.push(ch);
            num -= 1;
        }
        if num > 0 {
            heap.push((num, ch));
        }
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_diverse_string(1, 1, 7), "ccbccacc");
        debug_assert_eq!(longest_diverse_string(7, 1, 0), "aabaa");
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
