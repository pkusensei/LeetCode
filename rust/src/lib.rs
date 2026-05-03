mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(nums: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = nums.len();
    let mut closest = Vec::with_capacity(n);
    for i in 0..n {
        if i == 0 {
            closest.push(1);
        } else if i == n - 1 {
            closest.push(n - 2);
        } else {
            if nums[i].abs_diff(nums[i - 1]) <= nums[i].abs_diff(nums[1 + i]) {
                closest.push(i - 1);
            } else {
                closest.push(1 + i);
            }
        }
    }
    let mut fore_arr = vec![0; n];
    let mut prefix_fore = 0;
    for i in (0..n - 1).rev() {
        if closest[i] == 1 + i {
            prefix_fore += 1;
        } else {
            prefix_fore += (nums[i] - (nums[1 + i])).abs();
        }
        fore_arr[i] = prefix_fore;
    }
    let mut back_arr = vec![0; n];
    let mut prefix_back = 0;
    for i in 1..n {
        if closest[i] == i - 1 {
            prefix_back += 1;
        } else {
            prefix_back += nums[i] - nums[i - 1];
        }
        back_arr[i] = prefix_back;
    }
    let mut res = vec![];
    for q in queries {
        let [left, right] = [0, 1].map(|i| q[i] as usize);
        match left.cmp(&right) {
            std::cmp::Ordering::Equal => res.push(0),
            std::cmp::Ordering::Less => {
                let v1 = fore_arr[left] - fore_arr[right];
                let v2 = nums[right] - nums[left];
                res.push(v1.min(v2));
            }
            std::cmp::Ordering::Greater => {
                let v1 = back_arr[left] - back_arr[right];
                let v2 = nums[left] - nums[right];
                res.push(v1.min(v2));
            }
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
        assert_eq!(min_cost(&[-5, -2, 3], &[[0, 2], [2, 0], [1, 2]]), [6, 2, 5]);
        assert_eq!(
            min_cost(&[0, 2, 3, 9], &[[3, 0], [1, 2], [2, 0]]),
            [4, 1, 3]
        )
    }

    #[test]
    fn test() {
        assert_eq!(min_cost(&[-12, -4, 17, 24], &[[2, 0], [0, 2]]), [22, 22]);
    }
}
