mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_rabbits(answers: &[i32]) -> i32 {
    let nums = answers
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });
    let mut res = 0;
    for (k, v) in nums.into_iter() {
        if k + 1 >= v {
            res += k + 1;
        } else {
            let mut groups = v / (1 + k);
            if v % (1 + k) > 0 {
                groups += 1
            }
            res += groups * (1 + k);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_rabbits(&[1, 1, 2]), 5);
        debug_assert_eq!(num_rabbits(&[10, 10, 10]), 11);
    }

    #[test]
    fn test() {
        debug_assert_eq!(num_rabbits(&[1, 0, 1, 0, 0]), 5);
        debug_assert_eq!(num_rabbits(&[2, 1, 2, 2, 2, 2, 2, 2, 1, 1]), 13);
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
