mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length(nums: &[i32]) -> i32 {
    use itertools::Itertools;
    let freq = nums.iter().copied().counts();
    let mut res = freq
        .get(&1)
        .map(|&v| if v & 1 == 1 { v } else { v - 1 })
        .unwrap_or(1);
    for (&k, &v) in freq.iter() {
        if k == 1 || v == 1 {
            continue;
        }
        let mut num = k;
        let mut curr = 0;
        while let Some(&v) = freq.get(&num) {
            if v > 1 {
                curr += 2;
                let Some(next) = num.checked_mul(num) else {
                    break;
                };
                num = next;
            } else {
                curr += 1;
                break;
            }
        }
        if curr & 1 == 0 {
            curr -= 1;
        }
        res = res.max(curr)
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
        assert_eq!(maximum_length(&[5, 4, 1, 2, 2]), 3);
        assert_eq!(maximum_length(&[1, 3, 2, 4]), 1);
    }

    #[test]
    fn test() {}
}
