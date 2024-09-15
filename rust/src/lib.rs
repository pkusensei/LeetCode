mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn reconstruct_queue(mut people: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    // sort order: h increasing
    // with the same h, pop the one with smaller k first
    // so that [h, 0] gets inserted before [h, n]
    people.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let n = people.len();
    let mut res = Vec::with_capacity(n);
    while let Some([h, k]) = people.pop() {
        if res.is_empty() || k == 0 {
            res.insert(0, [h, k]);
            continue;
        }
        if (k as usize) >= res.len() {
            res.push([h, k]);
        } else {
            res.insert(k as usize, [h, k]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            reconstruct_queue(vec![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]),
            [[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
        );
        debug_assert_eq!(
            reconstruct_queue(vec![[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]),
            [[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: DerefMut<Target = [T1]>,
        I2: DerefMut<Target = [T2]>,
    {
        i1.sort_unstable();
        i2.sort_unstable();
        debug_assert_eq!(*i1, *i2);
    }
}
