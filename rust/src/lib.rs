mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_max_or_subsets(nums: &[i32]) -> i32 {
    let val = nums.iter().fold(0, |acc, n| acc | n);
    let space = 1 << nums.len(); // predetermined choise space 2**n
    let mut res = 0;
    for mask in 0..space {
        // Each mask is a preset selection of numbers
        let mut temp = 0;
        for (i, num) in nums.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                // current num is picked
                temp |= num;
                if temp == val {
                    res += 1;
                    break; // val is already the max temp could reach
                }
            }
        }
    }
    res
    // backtrack(nums, val, 0)
}

// smh this is faster
fn backtrack(nums: &[i32], val: i32, curr: i32) -> i32 {
    match nums {
        [] => (curr == val).into(),
        [head, tail @ ..] => backtrack(tail, val, curr) + backtrack(tail, val, curr | head),
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_max_or_subsets(&[3, 1]), 2);
        debug_assert_eq!(count_max_or_subsets(&[2, 2, 2]), 7);
        debug_assert_eq!(count_max_or_subsets(&[3, 2, 1, 5]), 6);
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
