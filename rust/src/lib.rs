mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut freq = vec![0; 1 + k as usize];
    for left in 0..n / 2 {
        let right = n - left - 1;
        let v = nums[left].abs_diff(nums[right]);
        freq[v as usize] += 1;
    }
    let [mut maxf1, mut maxf2] = [0, 0];
    let [mut x1, mut x2] = [0, 0];
    for (x, &f) in freq.iter().enumerate() {
        if f > maxf1 {
            (maxf2, x2) = (maxf1, x1);
            (maxf1, x1) = (f, x);
        } else if f > maxf2 {
            (maxf2, x2) = (f, x);
        }
    }
    solve(&nums, k, x1 as i32).min(solve(&nums, k, x2 as i32))
}

fn solve(nums: &[i32], k: i32, x: i32) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for left in 0..n / 2 {
        let right = n - left - 1;
        let a = nums[left].min(nums[right]);
        let b = nums[left].max(nums[right]);
        match (b - a).cmp(&x) {
            std::cmp::Ordering::Less => {
                if (0..=k).contains(&(a + x)) || (0..=k).contains(&(b - x)) {
                    res += 1;
                } else {
                    res += 2;
                }
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => res += 1,
        }
    }
    res
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
    fn basics() {
        assert_eq!(min_changes(vec![0, 1, 2, 3, 3, 6, 5, 4], 6), 2);
        assert_eq!(min_changes(vec![1, 0, 1, 2, 4, 3], 4), 2);
    }

    #[test]
    fn test() {
        assert_eq!(
            min_changes(vec![3, 1, 7, 7, 8, 7, 0, 5, 8, 0, 6, 7, 0, 2, 6, 6], 8),
            6
        );
    }
}
