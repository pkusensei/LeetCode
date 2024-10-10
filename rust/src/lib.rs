mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn judge_point24(mut cards: [i32; 4]) -> bool {
    cards.sort_unstable();
    loop {
        let nums: [f64; 4] = cards.map(f64::from);
        if dfs(&nums) {
            return true;
        }
        let Some(v) = next_permutation(cards) else {
            break;
        };
        cards = v;
    }
    false
}

fn dfs(nums: &[f64]) -> bool {
    if nums.len() == 1 {
        return (nums[0] - 24.0).abs() < 0.000_001;
    }
    for (idx, w) in nums.windows(2).enumerate() {
        let mut temp = nums.to_vec();
        temp.remove(1 + idx);
        for value in calc(w[0], w[1]) {
            temp[idx] = value;
            if dfs(&temp) {
                return true;
            }
        }
    }
    false
}

fn next_permutation(mut nums: [i32; 4]) -> Option<[i32; 4]> {
    // Find the largest index i such that a[i] < a[i + 1].
    // If no such index exists, the permutation is the last permutation.
    let Some(i) =
        nums.windows(2)
            .enumerate()
            .rev()
            .find_map(|(idx, w)| if w[0] < w[1] { Some(idx) } else { None })
    else {
        // nums.sort_unstable(); // [4,3,2,1] => [1,2,3,4] back to increasing
        return None;
    };

    // Find the largest index j, such that i < j && a[i] < a[j].
    if let Some(j) = nums.iter().enumerate().rev().find_map(|(idx, &v)| {
        if i < idx && nums[i] < v {
            Some(idx)
        } else {
            None
        }
    }) {
        nums.swap(i, j);
    }
    nums[i + 1..].reverse();
    Some(nums)
}

fn calc(a: f64, b: f64) -> Vec<f64> {
    let mut res = vec![a + b, a - b, a * b];
    if b != 0.0 {
        res.push(a / b);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(judge_point24([4, 1, 8, 7]));
        debug_assert!(!judge_point24([1, 2, 1, 2]));
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
