mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_time_from_digits(arr: [i32; 4]) -> String {
    let nums = [0, 1, 2, 3].map(|i| arr[i] as u8);
    let mut res = String::new();
    for bit in 0..4 {
        enumerate(nums, 1 << bit, &mut vec![nums[bit]], &mut res);
    }
    res
}

fn enumerate(nums: [u8; 4], mask: u8, curr: &mut Vec<u8>, record: &mut String) {
    if mask == 0b1111 {
        if let Some(s) = check(curr) {
            if s > *record {
                *record = s;
            }
        }
        return;
    }
    for bit in 0..4 {
        if (mask >> bit) & 1 == 0 {
            curr.push(nums[bit]);
            enumerate(nums, mask | (1 << bit), curr, record);
            curr.pop();
        }
    }
}

fn check(s: &[u8]) -> Option<String> {
    if s.len() != 4 {
        return None;
    }
    let (h, m) = s.split_at(2);
    let [h, m] = [h, m].map(|v| v[0] * 10 + v[1]);
    if (0..24).contains(&h) && (0..60).contains(&m) {
        Some(format!("{:02}:{:02}", h, m))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_time_from_digits([1, 2, 3, 4]), "23:41");
        debug_assert_eq!(largest_time_from_digits([5, 5, 5, 5]), "");
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
