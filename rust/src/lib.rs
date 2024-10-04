mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn least_interval(tasks: &[char], n: i32) -> i32 {
    // with_pq(tasks, n)
    let counts = tasks.iter().fold([0; 26], |mut acc, &ch| {
        acc[(u32::from(ch) - u32::from('A')) as usize] += 1;
        acc
    });
    let max = *counts.iter().max().unwrap();
    let max_count = counts.iter().filter(|&&n| n == max).count() as i32;
    ((max - 1) * (n + 1) + max_count).max(tasks.len() as i32)
    // ^^^^^^^ with gap/idle ^^^^^^^  vs  ^^^^^ no gap ^^^^^
    // Reagrding the gap situation
    // A cycle is (1+n) steps, with (max-1) cycles
    // And amount of max_count that's lingering
}

fn with_pq(tasks: &[char], n: i32) -> i32 {
    let mut chs: BinaryHeap<_> = tasks
        .iter()
        .fold(HashMap::with_capacity(26), |mut acc, &ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
        .into_values()
        .collect();

    let mut res = 0;
    let mut temp = Vec::with_capacity(1 + n as usize);
    while !chs.is_empty() {
        let mut cycle = n + 1;
        let mut count = 0;
        while cycle > 0 {
            cycle -= 1;
            let Some(v) = chs.pop() else {
                break;
            };
            if v > 1 {
                temp.push(v - 1);
            }
            count += 1;
        }
        chs.extend(temp.drain(..));
        res += if chs.is_empty() { count } else { n + 1 };
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(least_interval(&['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
        debug_assert_eq!(least_interval(&['A', 'C', 'A', 'B', 'D', 'B'], 1), 6);
        debug_assert_eq!(least_interval(&['A', 'A', 'A', 'B', 'B', 'B'], 3), 10);
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
