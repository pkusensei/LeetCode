mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let [mut pos, mut neg] = [0, 1].map(|_| vec![]);
    for &num in nums.iter() {
        if num > 0 {
            pos.push(num);
        } else {
            neg.push(num);
        }
    }
    pos.into_iter().zip(neg).flat_map(|(a, b)| [a, b]).collect()
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
