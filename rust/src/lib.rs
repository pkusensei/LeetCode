mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find132pattern(nums: &[i32]) -> bool {
    let n = nums.len();
    if n < 3 {
        return false;
    }
    // stores [i] => min until current index
    let mut mins = Vec::with_capacity(n);
    mins.push(nums[0]);
    for (i, &num) in nums.iter().enumerate().skip(1) {
        mins.push(num.min(mins[i - 1]));
    }
    // stores [k] => numbers bigger than [i] and to the right of [j]
    let mut stack = vec![];
    for (&num, &min) in nums.iter().zip(mins.iter()).skip(1).rev() {
        // [j] [i] pair in reverse order
        if num <= min {
            continue;
        }
        // As loop approaches left end of array
        // [i] might increase
        // So here it pops [k]s potentially smaller than [i]
        while stack.last().is_some_and(|&n| n <= min) {
            stack.pop();
        }
        // Now [i] < [k]
        // check if [k] < [j]
        if stack.last().is_some_and(|&n| n < num) {
            return true;
        }
        stack.push(num);
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!find132pattern(&[1, 2, 3, 4]));
        debug_assert!(find132pattern(&[3, 1, 4, 2]));
        debug_assert!(find132pattern(&[-1, 3, 2, 0]));
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
