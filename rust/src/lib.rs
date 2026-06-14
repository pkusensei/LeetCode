mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut res = 1;
    for left in 0..n {
        let mut freq = HashMap::from([(nums[left], 1)]);
        let mut max_f = 1;
        'out: for right in 1 + left..n {
            let v = freq.entry(nums[right]).or_insert(0);
            *v += 1;
            max_f = max_f.max(*v);
            if freq.len() == 1 {
                res = res.max(1 + right - left);
            } else if max_f & 1 == 0 {
                let mut seen = [false; 2];
                for &v in freq.values() {
                    if v == max_f {
                        seen[0] = true
                    } else if v == max_f / 2 {
                        seen[1] = true
                    } else {
                        continue 'out;
                    }
                }
                if seen[0] && seen[1] {
                    res = res.max(1 + right - left);
                }
            }
        }
    }
    res as i32
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
    fn basics() {}

    #[test]
    fn test() {}
}
