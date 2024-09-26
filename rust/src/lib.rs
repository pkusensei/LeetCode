mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn next_greater_elements(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut nums = nums.to_vec();
    nums.extend_from_within(..);
    let mut stack = vec![];
    let mut res = vec![None; n];
    for (idx, &num) in nums.iter().enumerate().rev() {
        if stack.is_empty() {
            stack.push(num);
            continue;
        }
        while stack.last().is_some_and(|&v| v <= num) {
            stack.pop();
        }
        if let (Some(last), Some(opt)) = (stack.last().copied(), res.get_mut(idx)) {
            opt.get_or_insert(last);
        }
        stack.push(num);
    }
    res.into_iter().map(|opt| opt.unwrap_or(-1)).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(next_greater_elements(&[1, 2, 1]), [2, -1, 2]);
        debug_assert_eq!(next_greater_elements(&[1, 2, 3, 4, 3]), [2, 3, 4, -1, 4]);
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
