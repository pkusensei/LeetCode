mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn split_array_same_average(nums: &[i32]) -> bool {
    let n = nums.len();
    if n == 1 {
        return false;
    }
    let sum: i32 = nums.iter().sum();
    let (mut pos, mut neg) = (vec![], vec![]);
    for num in nums.iter().map(|v| v * n as i32 - sum) {
        match num.signum() {
            0 => return true,
            1 => pos.push(num),
            -1 => neg.push(-num),
            _ => unreachable!(),
        }
    }
    mask(&pos).intersection(&mask(&neg)).next().is_some()
}

fn mask(nums: &[i32]) -> HashSet<i32> {
    let mut res = HashSet::new();
    let sum = nums.iter().sum();
    for &num in nums.iter() {
        let mut temp = res.clone();
        temp.insert(num);
        temp.extend(res.iter().map(|&v| v + sum));
        res = temp
    }
    res.remove(&sum);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(split_array_same_average(&[1, 2, 3, 4, 5, 6, 7, 8]));
        debug_assert!(!split_array_same_average(&[3, 1]));
    }

    #[test]
    fn test() {
        debug_assert!(!split_array_same_average(&[6, 8, 18, 3, 1]));
        debug_assert!(!split_array_same_average(&[
            60, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            30, 30, 30, 30, 30, 30, 30, 30
        ]));
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
