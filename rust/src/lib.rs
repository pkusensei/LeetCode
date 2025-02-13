mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_difference(nums: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let max = *nums.iter().max().unwrap_or(&0) as usize;
    let mut count = vec![0; 1 + max];
    let mut prefix = vec![count.clone()];
    for &num in nums.iter() {
        count[num as usize] += 1;
        prefix.push(count.clone());
    }
    let mut res = vec![];
    for q in queries.iter() {
        let [left, right] = q[..] else { unreachable!() };
        let count: Vec<_> = (0..=max)
            .map(|num| prefix[1 + right as usize][num] - prefix[left as usize][num])
            .collect();
        let mut prev = -1;
        let mut curr = i32::MAX;
        for (val, &c) in count.iter().enumerate() {
            if c == 0 {
                continue;
            }
            let val = val as i32;
            if prev > -1 && val > prev {
                curr = curr.min(val - prev);
            }
            prev = val;
        }
        res.push(if curr == i32::MAX { -1 } else { curr });
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
        assert_eq!(
            min_difference(&[1, 3, 4, 8], &[[0, 1], [1, 2], [2, 3], [0, 3]]),
            [2, 1, 4, 1]
        );
        assert_eq!(
            min_difference(&[4, 5, 2, 2, 7, 10], &[[2, 3], [0, 2], [0, 5], [3, 5]]),
            [-1, 1, 1, 3]
        );
    }

    #[test]
    fn test() {}
}
