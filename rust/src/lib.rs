mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = nums.clone();
    for (i, &num) in nums.iter().enumerate() {
        match num.cmp(&0) {
            std::cmp::Ordering::Less => {
                res[i] = nums[(i as i32 + num).rem_euclid(n as i32) as usize]
            }
            std::cmp::Ordering::Equal => res[i] = num,
            std::cmp::Ordering::Greater => res[i] = nums[(i + num as usize) % n],
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
    fn basics() {}

    #[test]
    fn test() {}
}
