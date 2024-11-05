mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use rand::Rng;

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    // merge_sort(&mut nums);
    // counting_sort(&mut nums);
    quicksort(&mut nums);
    nums
}

fn merge_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n == 1 {
        return;
    }
    merge_sort(&mut nums[..n / 2]);
    merge_sort(&mut nums[n / 2..]);
    let mut sorted = Vec::with_capacity(n);
    let (mut left, mut right) = (0, n / 2);
    while left < n / 2 && right < n {
        if nums[left] <= nums[right] {
            sorted.push(nums[left]);
            left += 1;
        } else {
            sorted.push(nums[right]);
            right += 1;
        }
    }
    sorted.extend_from_slice(&nums[left..n / 2]);
    sorted.extend_from_slice(&nums[right..]);
    nums.copy_from_slice(&sorted);
}

fn counting_sort(nums: &mut [i32]) {
    let (min, max) = nums
        .iter()
        .fold((i32::MAX, i32::MIN), |(curr_min, curr_max), &num| {
            (curr_min.min(num), curr_max.max(num))
        });
    let width = max - min;
    let mut counts = vec![0; 1 + width as usize];
    for &num in nums.iter() {
        counts[(num - min) as usize] += 1;
    }
    let mut idx = 0;
    for (delta, freq) in counts.into_iter().enumerate() {
        for _ in 0..freq {
            nums[idx] = min + delta as i32;
            idx += 1;
        }
    }
}

fn quicksort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    let pivot = rand::thread_rng().gen_range(0..nums.len());
    let val = nums[pivot];
    nums.swap(pivot, nums.len() - 1);
    let mut idx = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] <= val {
            nums.swap(i, idx);
            idx += 1;
        }
    }
    nums.swap(idx, nums.len() - 1);
    quicksort(&mut nums[..idx]);
    quicksort(&mut nums[1 + idx..]);
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(sort_array(vec![5, 2, 3, 1]), [1, 2, 3, 5]);
        debug_assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), [0, 0, 1, 1, 2, 5]);
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
