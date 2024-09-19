mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_arithmetic_slices(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    // raw dp
    // let mut dp = vec![vec![0; n]; n];
    // for i3 in 2..n {
    //     for i2 in 1..i3 {
    //         for i1 in 0..i2 {
    //             if i64::from(nums[i3]) - i64::from(nums[i2])
    //                 == i64::from(nums[i2]) - i64::from(nums[i1])
    //             {
    //                 dp[i3][i2] += 1 + dp[i2][i1];
    //                 res += 1 + dp[i2][i1]
    //             }
    //         }
    //     }
    // }

    use std::collections::HashMap;
    // for each number, produce a diff-count pair
    let mut seqs: Vec<HashMap<i32, i32>> = Vec::with_capacity(n);
    for (i, &x) in nums.iter().enumerate() {
        let mut curr = HashMap::new();
        for (j, &y) in nums.iter().take(i).enumerate() {
            let Some(diff) = x.checked_sub(y) else {
                continue;
            };
            let count = seqs[j].get(&diff).copied().unwrap_or(0);
            res += count;
            if x.checked_add(diff).is_some() {
                // presume a number exists as in y - diff - x - diff - num???
                // When a promising value visits this diff-count pair
                // the count is directly added to result ( res+=count )
                *curr.entry(diff).or_insert(0) += count + 1
            }
        }
        seqs.push(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(number_of_arithmetic_slices(&[2, 4, 6, 8, 10]), 7);
        debug_assert_eq!(number_of_arithmetic_slices(&[7, 7, 7, 7, 7]), 16);
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
