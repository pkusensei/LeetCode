mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let (n, k) = (nums.len(), k as usize);
    let mut curr = nums[..k]
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        });
    let mut temp = nums.iter().take(k).map(|&v| i64::from(v)).sum();
    let mut res = if curr.len() == k { temp } else { 0 };
    for idx in 1..=n - k {
        let del = nums[idx - 1];
        let add = nums[idx + k - 1];
        *curr.entry(del).or_insert(0) -= 1;
        *curr.entry(add).or_insert(0) += 1;
        if curr[&del] == 0 {
            curr.remove(&del);
        }
        temp -= i64::from(del);
        temp += i64::from(add);
        if curr.len() == k {
            res = res.max(temp)
        }
    }
    res
}

fn solve(nums: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let (mut res, mut sum) = (0, 0);
    let mut left = 0;
    let mut window = std::collections::HashMap::new();
    for (right, &num) in nums.iter().enumerate() {
        let last_occurence = window.get(&num);
        while last_occurence.is_some_and(|&v| v >= left) || right - left + 1 > k {
            sum -= nums[left];
            left += 1;
        }
        window.insert(num, right);
        sum += num;
        if right - left + 1 == k {
            res = res.max(sum);
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
        debug_assert_eq!(solve(&[1, 5, 4, 2, 9, 9, 9], 3), 15);
        debug_assert_eq!(solve(&[4, 4, 4], 3), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(solve(&[1, 1, 1, 7, 8, 9], 3), 24);
        debug_assert_eq!(solve(&[9, 9, 9, 1, 2, 3], 3), 12);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
