mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_pairs(nums: &mut [i32], k: i32) -> i32 {
    // nums.sort_unstable();
    // let mut res = std::collections::HashSet::new();
    // for (i, &num) in nums.iter().enumerate() {
    //     if let Ok(j) = nums[1 + i..].binary_search(&(num + k)) {
    //         res.insert([num, nums[j + i + 1]]);
    //     }
    // }
    // res.len() as _
    let dict = nums
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
    let mut res = 0;
    for (&num, &count) in dict.iter() {
        if dict.contains_key(&(num + k)) {
            if k > 0 {
                res += 1;
            }
            if k == 0 && count > 1 {
                res += 1
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_pairs(&mut [3, 1, 4, 1, 5], 2), 2);
        debug_assert_eq!(find_pairs(&mut [1, 2, 3, 4, 5], 1), 4);
        debug_assert_eq!(find_pairs(&mut [1, 3, 1, 5, 4], 0), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_pairs(&mut [-1, 0, 0, 1, 0, 0, -1], 1), 2);
        debug_assert_eq!(find_pairs(&mut [1, 1, 1, 2, 2], 1), 1);
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
