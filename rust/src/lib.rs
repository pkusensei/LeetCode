mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut seen = [false; 101];
    let [mut min, mut max] = [100, 1];
    for &num in nums.iter() {
        min = min.min(num);
        max = max.max(num);
        seen[num as usize] = true;
    }
    seen[min as usize..max as usize]
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if !v { Some((i as i32 + min)) } else { None })
        .collect()
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
