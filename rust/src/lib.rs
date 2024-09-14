mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn valid_utf8(data: &[i32]) -> bool {
    let n = data.len();
    let mut idx = 0;
    while idx < n {
        if data[idx] <= 0b0111_1111 {
            idx += 1
        } else if (data[idx] >> 3) == 0b1_1110 {
            if !is_valid(&data[idx + 1..], 3) {
                return false;
            }
            idx += 4
        } else if (data[idx] >> 4) == 0b1110 {
            if !is_valid(&data[idx + 1..], 2) {
                return false;
            }
            idx += 3;
        } else if (data[idx] >> 5) == 0b0110 {
            if !is_valid(&data[idx + 1..], 1) {
                return false;
            }
            idx += 2;
        } else {
            return false;
        }
    }
    true
}

fn is_valid(nums: &[i32], count: usize) -> bool {
    nums.len() >= count && nums.iter().take(count).all(|n| (n >> 6) == 0b0010)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(valid_utf8(&[197, 130, 1]));
        debug_assert!(!valid_utf8(&[235, 140, 4]));
    }

    #[test]
    fn test() {
        debug_assert!(!valid_utf8(&[248, 130, 130, 130]));
        debug_assert!(!valid_utf8(&[197, 194, 1]));
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
