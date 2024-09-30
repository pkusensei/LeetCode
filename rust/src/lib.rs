mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn next_greater_element(mut n: i32) -> i32 {
    let mut digits = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    let Some(digits) = next_permutation(digits) else {
        return -1;
    };
    let mut res: i32 = 0;
    for digit in digits.into_iter() {
        if let Some(v) = res.checked_mul(10).and_then(|n| n.checked_add(digit)) {
            res = v
        } else {
            return -1;
        }
    }
    res
}

fn next_permutation(mut nums: Vec<i32>) -> Option<Vec<i32>> {
    // find largest i such that a[i]<a[i+1]
    let i =
        nums.windows(2)
            .enumerate()
            .rev()
            .find_map(|(idx, w)| if w[0] < w[1] { Some(idx) } else { None })?;
    // find largest j such that ..i..j.. and a[i]<a[j]
    if let Some(j) =
        nums.iter().enumerate().rev().find_map(
            |(j, &n)| {
                if i < j && nums[i] < n {
                    Some(j)
                } else {
                    None
                }
            },
        )
    {
        nums.swap(i, j);
    }
    nums[i + 1..].reverse();
    Some(nums)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(next_greater_element(12), 21);
        debug_assert_eq!(next_greater_element(21), -1);
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
