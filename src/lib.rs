// https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
pub fn next_permutation(nums: &mut [i32]) {
    // Find the largest index i such that a[i] < a[i + 1].
    // If no such index exists, the permutation is the last permutation.
    let Some(i) =
        nums.windows(2)
            .enumerate()
            .rev()
            .find_map(|(idx, w)| if w[0] < w[1] { Some(idx) } else { None })
    else {
        nums.sort_unstable();
        return;
    };

    // Find the largest index j, such that i < j && a[i] < a[j].
    if let Some(j) = nums.iter().enumerate().rev().find_map(|(idx, v)| {
        if i < idx && nums[i] < *v {
            Some(idx)
        } else {
            None
        }
    }) {
        nums.swap(i, j);
    }
    nums[i + 1..].reverse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut nums = vec![1, 2, 3];
            next_permutation(&mut nums);
            debug_assert_eq!(nums, [1, 3, 2]);
        }
        {
            let mut nums = vec![3, 2, 1];
            next_permutation(&mut nums);
            debug_assert_eq!(nums, [1, 2, 3]);
        }
        {
            let mut nums = vec![1, 1, 5];
            next_permutation(&mut nums);
            debug_assert_eq!(nums, [1, 5, 1]);
        }
    }

    #[test]
    fn test() {}
}
