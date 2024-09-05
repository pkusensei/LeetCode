mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn missing_rolls(rolls: &[i32], mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let sum: i32 = rolls.iter().sum();
    let missing = mean * m - sum + mean * n;
    if missing < n {
        return vec![];
    }
    let average = missing / n;
    let remainder = missing % n;
    match (average, remainder) {
        (7.., _) => vec![],
        (6, 0) => vec![6; n as usize],
        (6, _) => vec![],
        _ => {
            let mut res = vec![average; n as usize];
            for i in 0..remainder {
                res[i as usize] += 1
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(missing_rolls(&[3, 2, 4, 3], 4, 2), [6, 6]);
        debug_assert_eq!(missing_rolls(&[1, 5, 6], 3, 4), [3, 2, 2, 2]);
        debug_assert!(missing_rolls(&[1, 2, 3, 4], 6, 4).is_empty());
    }

    #[test]
    fn test() {
        debug_assert!(missing_rolls(&[6, 3, 4, 3, 5, 3], 1, 6).is_empty());
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
