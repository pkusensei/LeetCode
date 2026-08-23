mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn valid_subarrays(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = nums.len();
    let max = *nums.iter().max().unwrap();
    let b = n.isqrt() as i32;
    let qn = queries.len();
    // (i, left, right)
    let sorted = queries
        .iter()
        .enumerate()
        .map(|(i, q)| (i, q[0], q[1]))
        .sorted_unstable_by(|q1, q2| {
            (q1.1 / b)
                .cmp(&(q2.1 / b))
                .then_with(|| (q1.2 / b).cmp(&(q2.2 / b)))
        });
    let mut freq = vec![0; 1 + max as usize];
    let [mut distinct, mut odd] = [0, 0];
    let mut p_left = 0;
    let mut p_right = -1;
    let mut res = vec![false; qn];
    for (i, left, right) in sorted {
        if (1 + right - left) & 1 == 1 {
            continue;
        }
        while left < p_left {
            p_left -= 1;
            add(&mut freq, nums[p_left as usize], &mut distinct, &mut odd);
        }
        while p_left < left {
            remove(&mut freq, nums[p_left as usize], &mut distinct, &mut odd);
            p_left += 1;
        }
        while p_right < right {
            p_right += 1;
            add(&mut freq, nums[p_right as usize], &mut distinct, &mut odd);
        }
        while right < p_right {
            remove(&mut freq, nums[p_right as usize], &mut distinct, &mut odd);
            p_right -= 1;
        }
        res[i] = distinct == k && odd == 0;
    }
    res
}

fn add(freq: &mut [i32], num: i32, distinct: &mut i32, odd: &mut i32) {
    freq[num as usize] += 1;
    if freq[num as usize] == 1 {
        *distinct += 1
    }
    if freq[num as usize] & 1 == 1 {
        *odd += 1
    } else {
        *odd -= 1
    }
}

fn remove(freq: &mut [i32], num: i32, distinct: &mut i32, odd: &mut i32) {
    freq[num as usize] -= 1;
    if freq[num as usize] == 0 {
        *distinct -= 1
    }
    if freq[num as usize] & 1 == 1 {
        *odd += 1
    } else {
        *odd -= 1
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
