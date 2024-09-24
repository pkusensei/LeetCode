mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn predict_the_winner(nums: &[i32]) -> bool {
    dfs(nums, 0, 0, true)
}

fn dfs(nums: &[i32], p1: i32, p2: i32, turn1: bool) -> bool {
    match nums {
        [] => p1 >= p2,
        &[num] => {
            if turn1 {
                p1 + num >= p2
            } else {
                p1 >= p2 + num
            }
        }
        &[first, .., last] => {
            if turn1 {
                let a = dfs(&nums[1..], p1 + first, p2, !turn1);
                let b = dfs(&nums[..nums.len() - 1], p1 + last, p2, !turn1);
                a || b
            } else {
                let a = dfs(&nums[1..], p1, p2 + first, !turn1);
                let b = dfs(&nums[..nums.len() - 1], p1, p2 + last, !turn1);
                a && b
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!predict_the_winner(&[1, 5, 2]));
        debug_assert!(predict_the_winner(&[1, 5, 233, 7]));
        debug_assert!(!predict_the_winner(&[1, 3, 1]));
    }

    #[test]
    fn test() {
        debug_assert!(predict_the_winner(&[1, 567, 1, 1]));
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
