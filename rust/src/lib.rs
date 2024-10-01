mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_arrange(arr: &[i32], k: i32) -> bool {
    // let counts = arr
    //     .iter()
    //     .fold(std::collections::HashMap::new(), |mut acc, &num| {
    //         *acc.entry((num % k + k) % k).or_insert(0) += 1;
    //         acc
    //     });
    // for &num in arr.iter() {
    //     let remainder = (num % k + k) % k;
    //     if remainder == 0 {
    //         if counts.get(&0).is_some_and(|v| v & 1 == 1) {
    //             return false;
    //         }
    //     } else if counts.get(&remainder) != counts.get(&(k - remainder)) {
    //         return false;
    //     }
    // }
    let counts = arr.iter().fold(vec![0; k as usize], |mut acc, &num| {
        acc[((num % k + k) % k) as usize] += 1;
        acc
    });
    if counts[0] & 1 > 0 {
        return false;
    }
    for i in 1..=k / 2 {
        if counts[i as usize] != counts[(k - i) as usize] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_arrange(&[1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5));
        debug_assert!(can_arrange(&[1, 2, 3, 4, 5, 6], 7));
        debug_assert!(!can_arrange(&[1, 2, 3, 4, 5, 6], 10));
    }

    #[test]
    fn test() {
        debug_assert!(!can_arrange(&[5, 5, 1, 2, 3, 4], 10));
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
