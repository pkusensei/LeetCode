mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn iterative(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; 15];
    let mut done = [false; 15];
    let mut groups = vec![nums];
    for bit in (0..15).rev() {
        let mut next = vec![];
        // Updated `done` could impact
        // further processing of `groups` in the same outer loop
        // if done[bit] {continue;}
        for g in groups {
            if done[bit] {
                next.push(g);
                continue;
            }
            let [mut left, mut right] = [const { vec![] }; 2];
            for num in g {
                if num & (1 << bit) > 0 {
                    left.push(num);
                } else {
                    right.push(num);
                }
            }
            if !left.is_empty() {
                res[14 - bit] += left.len() as i32;
                next.push(left);
            }
            if !right.is_empty() {
                done[bit] = true;
                next.push(right);
            }
        }
        groups = next;
    }
    res
}

pub fn largest_power(nums: &[i32]) -> Vec<i32> {
    let mut res = vec![0; 15];
    let mut done = [false; 15];
    dfs(&nums, 14, &mut res, &mut done);
    res
}

fn dfs(nums: &[i32], bit: usize, res: &mut [i32], done: &mut [bool; 15]) {
    if bit >= 15 {
        return;
    }
    if done[bit] {
        dfs(nums, bit.wrapping_sub(1), res, done);
        return;
    }
    let [mut left, mut right] = [const { vec![] }; 2];
    for &num in nums {
        if num & (1 << bit) > 0 {
            left.push(num);
        } else {
            right.push(num);
        }
    }
    if !left.is_empty() {
        res[14 - bit] += left.len() as i32;
        dfs(&left, bit.wrapping_sub(1), res, done);
    }
    if !right.is_empty() {
        done[bit] = true;
        dfs(&right, bit.wrapping_sub(1), res, done);
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
    fn basics() {
        assert_eq!(
            iterative(vec![5, 2]),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            iterative(vec![7, 5]),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2]
        );

        assert_eq!(
            largest_power(&[5, 2]),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            largest_power(&[7, 5]),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2]
        );
    }

    #[test]
    fn test() {}
}
