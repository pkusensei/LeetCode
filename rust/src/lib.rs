mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (n1, n2) = (nums1.len(), nums2.len());
    let [mut i1, mut i2] = [0, 0];
    let mut res = vec![];
    while i1 < n1 && i2 < n2 {
        match nums1[i1][0].cmp(&nums2[i2][0]) {
            std::cmp::Ordering::Less => {
                res.push(nums1[i1].clone());
                i1 += 1;
            }
            std::cmp::Ordering::Equal => {
                res.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
                i1 += 1;
                i2 += 1;
            }
            std::cmp::Ordering::Greater => {
                res.push(nums2[i2].clone());
                i2 += 1;
            }
        }
    }
    while i1 < n1 {
        res.push(nums1[i1].clone());
        i1 += 1;
    }
    while i2 < n2 {
        res.push(nums2[i2].clone());
        i2 += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
