mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let n = nums.len();
    let [l, r] = [l, r].map(|v| v as usize);
    let prefix = nums
        .iter()
        .take(r)
        .fold(Vec::with_capacity(r), |mut acc, &v| {
            acc.push(v + acc.last().unwrap_or(&0));
            acc
        });
    let mut res = None::<i32>;
    for len in l..=r {
        let mut curr = prefix[len - 1];
        if curr > 0 {
            if let Some(v) = res {
                res = Some(v.min(curr));
            } else {
                res = Some(curr)
            }
        }
        for i in len..n {
            curr += nums[i];
            curr -= nums[i - len];
            if curr > 0 {
                if let Some(v) = res {
                    res = Some(v.min(curr));
                } else {
                    res = Some(curr)
                }
            }
        }
    }
    res.unwrap_or(-1)
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
    fn basics() {}

    #[test]
    fn test() {}
}
